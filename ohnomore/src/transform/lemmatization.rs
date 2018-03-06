//! Lemmatization transformations.
//!
//! This module provides transformations that converts lemmas to TüBa-D/Z-style
//! lemmas.

use std::collections::HashMap;
use std::io::BufRead;

use fst::{Set, SetBuilder};
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

use constants::*;
use error::*;
use transform::auxpassiv::{ancestor_path, verb_lemma_tag, VerbLemmaTag};
use transform::{DependencyGraph, Token, Transform};
use transform::svp::longest_prefixes;

/// Add auxililary/passive markers.
///
/// This transformation adds auxiliary/passive markers to auxiliary/passive
/// verbs. Verbs such as *sein* both have auxiliary and other readings. E.g.
///
/// * *Ich hab einen traum* (non-auxiliary)
/// * *Ich hab geträumt* (auxiliary)
///
/// The TüBa-D/Z marks auxiliary and passive readings using the *%aux* and
/// *%passiv* suffixes (e.g. *haben%aux* and *werden%passiv*).
///
/// This transformation adds such markers. It distinguishes auxiliary/passive
/// readings from other readings using dependency structure.
pub struct AddAuxPassivTag;

impl<T> Transform<T> for AddAuxPassivTag
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String {
        let token = &graph[node];
        let lemma = token.lemma();

        // The auxiliary tag only applies to auxiliaries and modals.
        if !token.tag().starts_with(AUXILIARY_PREFIX) && !token.tag().starts_with(MODAL_PREFIX) {
            return lemma.to_owned();
        }

        // Find the verb's lemma tag.
        match verb_lemma_tag(graph, node) {
            VerbLemmaTag::Auxiliary => return format!("{}{}", lemma, AUXILIARY_MARKER),
            VerbLemmaTag::Passive => return format!("{}{}", lemma, PASSIVE_MARKER),
            VerbLemmaTag::None => (),
        }

        // If the verb lemma tag was none, it could be that a verb
        // is indirectly attached to the auxiliary through conjunction.
        //
        // E.g.: "Sie wollen und werden ein Schauspielhaus fertigstellen."
        if graph
            .edges_directed(node, Direction::Outgoing)
            .find(|e| e.weight() != PUNCTUATION_RELATION)
            .is_none()
        {
            if let Some(node) = ancestor_path(
                graph,
                node,
                &[CONJ_COMPLEMENT_RELATION, COORDINATION_RELATION],
            ) {
                match verb_lemma_tag(graph, node) {
                    VerbLemmaTag::Auxiliary => return format!("{}{}", lemma, AUXILIARY_MARKER),
                    VerbLemmaTag::Passive => return format!("{}{}", lemma, PASSIVE_MARKER),
                    VerbLemmaTag::None => (),
                }
            }
        }

        // There are no outgoing edges with the auxiliary relation.
        // Check that this verb is not an adverbial modification of
        // an infinitive.
        for edge in graph.edges_directed(node, Direction::Incoming) {
            let head_tag = graph[edge.source()].tag();
            if edge.weight() == ADVERBIAL_RELATION
                && (head_tag == INFINITIVE_VERB_TAG || head_tag == PARTICIPLE_TAG)
            {
                return format!("{}{}", lemma, AUXILIARY_MARKER);
            }
        }

        lemma.to_owned()
    }
}

/// Add separable verb prefixes to verbs.
///
/// TüBa-D/Z marks separable verb prefixes in the verb lemma. E.g. *ab#zeichnen*,
/// where *ab* is the separable prefix. This transformation handles cases where
/// the prefix is separated from the verb. For example, in the sentence
///
/// *Diese änderungen zeichnen sich bereits ab .*
///
/// The transformation rule will lemmatize *zeichnen* to *ab#zeichnen*. The
/// separable particle of a verb is found using dependency structure. In some
/// limited cases, it will also handle verbs with multiple `competing' separable
/// prefixes. For example, *nimmt* in
///
/// *[...] nimmt eher zu als ab*
///
/// is lemmatized as *zu#nehmen|ab#nehmen*.
pub struct AddSeparatedVerbPrefix {
    multiple_prefixes: bool,
}

impl AddSeparatedVerbPrefix {
    pub fn new(multiple_prefixes: bool) -> Self {
        AddSeparatedVerbPrefix { multiple_prefixes }
    }
}

impl<T> Transform<T> for AddSeparatedVerbPrefix
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String {
        let token = &graph[node];
        let lemma = token.lemma();

        if !is_separable_verb(token.tag()) {
            return lemma.to_owned();
        }

        let mut lemma = lemma.to_owned();

        // Find all nodes that are attached with the separable verb dependency
        // relation.
        //
        // Fixme: check AVZ/KON relation as well?
        // Fixme: what about particles linked KON?
        let mut prefix_iter = graph
            .edges_directed(node, Direction::Outgoing)
            .filter(|e| graph[e.target()].tag() == SEPARABLE_PARTICLE_POS);

        if self.multiple_prefixes {
            let mut lemmas = Vec::new();

            // Fixme: prefixes are not returned in sentence order?
            for edge in prefix_iter {
                let prefix = &graph[edge.target()];
                lemmas.push(format!("{}#{}", prefix.form().to_lowercase(), lemma));
            }

            if lemmas.is_empty() {
                lemma
            } else {
                lemmas.join("|")
            }
        } else {
            if let Some(edge) = prefix_iter.next() {
                let prefix = &graph[edge.target()];
                lemma.insert_str(0, &format!("{}#", prefix.form().to_lowercase()));
            }

            lemma
        }
    }
}

/// Lemmatize tokens where the form is the lemma.
pub struct FormAsLemma;

impl<T> Transform<T> for FormAsLemma
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String {
        let token = &graph[node];

        // Handle tags for which the lemma is the lowercased form.
        if LEMMA_IS_FORM_TAGS.contains(token.tag()) {
            token.form().to_lowercase()
        } else {
            token.lemma().to_owned()
        }
    }
}

/// Mark separable verb prefixes in verbs.
///
/// TüBa-D/Z marks separable verb prefixes in the verb lemma. E.g. *ab#zeichnen*,
/// where *ab* is the separable prefix. This transformation handles cases where
/// the prefix is **not** separated from the verb. For example, it makes the
/// following transformations:
///
/// 1. *abhing/hängen* -> *abhängen*
/// 2. *dazugefügt/fügen* -> *dazu#fügen*
/// 3. *wiedergutgemacht/machen* -> *wieder#gut#machen*
/// 4. *hinzubewegen/bewegen* -> *hin#bewegen*
///
/// The transformation rule prefers analysis with longer prefixes over shorter
/// prefixes. This leads to the analysis (2) rather than *da#zu#fügen*.
///
/// When a verb contains multiple separable prefixes, this transformation rule
/// attempts to find them, as in (3).
///
/// In 'zu'-infinitives *zu* is removed and not analyzed as being (part of) a
/// separable prefix.
pub struct MarkVerbPrefix {
    prefix_verbs: HashMap<String, String>,
    prefixes: Set,
}

impl MarkVerbPrefix {
    /// Create this transformation. A simple lookup for prefix verbs can be
    /// provided. More crucially, a set of prefixes must be provided to find
    /// prefixes.
    pub fn new(prefix_verbs: HashMap<String, String>, prefixes: Set) -> Self {
        MarkVerbPrefix {
            prefix_verbs,
            prefixes,
        }
    }

    pub fn set_prefix_verbs(&mut self, prefix_verbs: HashMap<String, String>) {
        self.prefix_verbs = prefix_verbs;
    }
}

impl<T> Transform<T> for MarkVerbPrefix
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String {
        let token = &graph[node];
        let lemma = token.lemma();
        let lemma_lc = lemma.to_lowercase();

        if !is_verb(token.tag()) {
            return lemma.to_owned();
        }

        // There are two cases that we have to handle separately:
        //
        // 1. The lemmatizer did not strip the prefix. In this case, we
        //    perform a lemma lookup. For now, removing prefixes from the
        //    lemma itself seems to be too tricky.
        //
        // 2. The lemmatizer stripped the prefix. The prefix needs to be
        //    inferred from the token's form.

        // Case 1: try a simple lookup for the lemma
        if let Some(sep_lemma) = self.prefix_verbs.get(&lemma_lc) {
            return sep_lemma.clone();
        }

        // Case 2: there are no prefixes in the lemma, try to find prefixes
        // in the form.
        let form_lc = token.form().to_lowercase();
        let mut lemma_parts = longest_prefixes(&self.prefixes, &form_lc, &lemma_lc, token.tag());
        if !lemma_parts.is_empty() {
            lemma_parts.push(lemma_lc.clone());
            return lemma_parts.join("#");
        }

        lemma.to_owned()
    }
}

pub trait ReadVerbPrefixes {
    fn read_verb_prefixes<R>(r: R) -> Result<MarkVerbPrefix>
    where
        R: BufRead;
}

impl ReadVerbPrefixes for MarkVerbPrefix {
    fn read_verb_prefixes<R>(r: R) -> Result<MarkVerbPrefix>
    where
        R: BufRead,
    {
        let mut builder = SetBuilder::memory();

        for line in r.lines() {
            let line = line?;

            builder.insert(&line)?;
        }

        let bytes = builder.into_inner()?;
        let prefixes = Set::from_bytes(bytes)?;

        Ok(MarkVerbPrefix::new(HashMap::new(), prefixes))
    }
}

pub struct RestoreCase;

impl<T> Transform<T> for RestoreCase
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String {
        let token = &graph[node];

        if token.tag() == NOUN_TAG {
            uppercase_first_char(token.lemma())
        } else {
            token.lemma().to_owned()
        }
    }
}

fn uppercase_first_char<S>(s: S) -> String
where
    S: AsRef<str>,
{
    // Hold your seats... This is a bit convoluted, because uppercasing a
    // unicode codepoint can result in multiple codepoints. Although this
    // should not hapen in German orthography, we want to be correct here...

    let mut chars = s.as_ref().chars();
    let first = ok_or!(chars.next(), return String::new());

    first.to_uppercase().chain(chars).collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufReader;
    use std::iter::FromIterator;

    use transform::test_helpers::run_test_cases;

    use super::{uppercase_first_char, AddAuxPassivTag, AddSeparatedVerbPrefix, FormAsLemma,
                MarkVerbPrefix, ReadVerbPrefixes, RestoreCase};

    #[test]
    pub fn first_char_is_uppercased() {
        assert_eq!(uppercase_first_char("test"), "Test");
        assert_eq!(uppercase_first_char("Test"), "Test");
        assert_eq!(uppercase_first_char(""), "");
    }

    #[test]
    pub fn add_aux_passiv_tag() {
        run_test_cases("testdata/add-aux-passiv-tag.test", AddAuxPassivTag);
    }

    #[test]
    pub fn add_separated_verb_prefix() {
        run_test_cases(
            "testdata/add-separated-verb-prefix.test",
            AddSeparatedVerbPrefix {
                multiple_prefixes: true,
            },
        );
    }

    #[test]
    pub fn form_as_lemma() {
        run_test_cases("testdata/form-as-lemma.test", FormAsLemma);
    }

    #[test]
    pub fn mark_verb_prefix() {
        let prefix_verbs = HashMap::from_iter(vec![
            (String::from("abbestellen"), String::from("ab#bestellen")),
        ]);

        let reader = BufReader::new(File::open("data/tdz10-separable-prefixes.txt").unwrap());
        let mut transform = MarkVerbPrefix::read_verb_prefixes(reader).unwrap();
        transform.set_prefix_verbs(prefix_verbs);

        run_test_cases("testdata/mark-verb-prefix.test", transform);
    }

    #[test]
    pub fn restore_case() {
        run_test_cases("testdata/restore-case.test", RestoreCase);
    }
}
