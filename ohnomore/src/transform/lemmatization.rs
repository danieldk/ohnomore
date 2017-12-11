use std::collections::HashMap;

use fst::{IntoStreamer, Set, Streamer};
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

use constants::*;

use automaton::PrefixAutomaton;
use transform::{DependencyGraph, Token, Transform};

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

        match graph.edges_directed(node, Direction::Outgoing).find(|e| {
            e.weight() == AUXILIARY_RELATION
        }) {
            Some(edge) => {
                if lemma == PASSIVE_VERB_LEMMA && graph[edge.target()].tag() == PARTICIPLE_TAG {
                    format!("{}{}", lemma, PASSIVE_MARKER)
                } else {
                    format!("{}{}", lemma, AUXILIARY_MARKER)
                }
            }
            None => lemma.to_owned(),
        }
    }
}

pub struct AddSeparatedVerbPrefix {
    multiple_prefixes: bool,
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
        let mut prefix_iter = graph.edges_directed(node, Direction::Outgoing).filter(
            |e| {
                graph[e.target()].tag() == SEPARABLE_PARTICLE_POS
            },
        );

        if self.multiple_prefixes {
            let mut lemmas = Vec::new();

            // Fixme: prefixes are not returned in sentence order?
            for edge in prefix_iter {
                let prefix = &graph[edge.target()];
                lemmas.push(format!("{}#{}", prefix.lemma(), lemma));
            }

            lemmas.join("|")
        } else {
            if let Some(edge) = prefix_iter.next() {
                let prefix = &graph[edge.target()];
                lemma.insert_str(0, &format!("{}#", prefix.lemma()));
            }

            lemma
        }
    }
}

pub struct MarkVerbPrefix {
    prefix_verbs: HashMap<String, String>,
    prefixes: Set,
}

impl MarkVerbPrefix {
    pub fn new(prefix_verbs: HashMap<String, String>, prefixes: Set) -> Self {
        MarkVerbPrefix {
            prefix_verbs,
            prefixes,
        }
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
        let mut lemma_parts = longest_prefixes(&self.prefixes, &form_lc, &lemma_lc);
        if !lemma_parts.is_empty() {
            // abzuarbeiten arbeiten -> ab#arbeiten, not: ab#zu#arbeiten
            if token.tag() == ZU_INFINITIVE_VERB && lemma_parts.last().unwrap() == "zu" {
                lemma_parts.pop();
            }

            lemma_parts.push(lemma_lc.clone());
            return lemma_parts.join("#");
        }

        lemma.to_owned()
    }
}

fn longest_prefixes<F, L>(prefix_set: &Set, form: F, lemma: L) -> Vec<String>
where
    F: AsRef<str>,
    L: AsRef<str>,
{
    let lemma = lemma.as_ref();
    let mut stripped = form.as_ref();

    let mut prefixes = Vec::new();
    while let Some(prefix) = longest_prefix(prefix_set, &stripped) {
        // Prefix should not end with lemma. E.g.:
        // abgefangen fangen -> ab#fangen, not: ab#gefangen#fangen
        if lemma.starts_with(&prefix) || prefix.ends_with(&lemma) ||
            !is_verb(&stripped[prefix.len()..])
        {
            break;
        }

        stripped = &stripped[prefix.len()..];
        prefixes.push(prefix);
    }

    prefixes
}

fn is_verb<S>(verb: S) -> bool
where
    S: AsRef<str>,
{
    // A separable verb with a length shorter than 3 is unlikely.
    verb.as_ref().len() > 2
}

fn longest_prefix<S>(prefix_set: &Set, s: S) -> Option<String>
where
    S: AsRef<str>,
{
    let automaton = PrefixAutomaton::from(s.as_ref());

    let mut prefix_candidates = Vec::new();

    let mut stream = prefix_set.search(&automaton).into_stream();
    while let Some(prefix) = stream.next() {
        prefix_candidates.push(prefix.to_owned());
    }

    let longest_prefix = prefix_candidates.into_iter().max_by_key(|p| p.len());

    longest_prefix.map(|p| {
        String::from_utf8(p).expect(
            "Cannot decode prefix, PrefixAutomaton returned invalid prefix",
        )
    })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::iter::FromIterator;

    use fst::{Set, SetBuilder};

    use error::*;
    use transform::test_helpers::run_test_cases;

    use super::{AddAuxPassivTag, AddSeparatedVerbPrefix, MarkVerbPrefix};

    #[test]
    pub fn add_aux_passiv_tag() {
        run_test_cases("testdata/add-aux-passiv-tag.test", AddAuxPassivTag);
    }

    #[test]
    pub fn add_separated_verb_prefix() {
        run_test_cases(
            "testdata/add-separated-verb-prefix.test",
            AddSeparatedVerbPrefix { multiple_prefixes: true },
        );
    }

    #[test]
    pub fn mark_verb_prefix() {
        let prefix_verbs = HashMap::from_iter(vec![
            (
                String::from("abbestellen"),
                String::from("ab#bestellen")
            ),
        ]);
        let reader = BufReader::new(File::open("data/tdz10-separable-prefixes.txt").unwrap());
        let prefixes = read_prefixes(reader).unwrap();

        run_test_cases(
            "testdata/mark-verb-prefix.test",
            MarkVerbPrefix {
                prefix_verbs,
                prefixes,
            },
        );
    }

    #[test]
    pub fn mark_verb_prefix2() {
        let prefix_verbs = HashMap::from_iter(vec![
            (
                String::from("abbestellen"),
                String::from("ab#bestellen")
            ),
        ]);
        let reader = BufReader::new(File::open("data/tdz10-separable-prefixes.txt").unwrap());
        let prefixes = read_prefixes(reader).unwrap();

        run_test_cases(
            "testdata/mark-test-cases.test",
            MarkVerbPrefix {
                prefix_verbs,
                prefixes,
            },
        );
    }

    fn read_prefixes<R>(r: R) -> Result<Set>
    where
        R: BufRead,
    {
        let mut builder = SetBuilder::memory();

        for line in r.lines() {
            let line = line?;

            builder.insert(&line)?;
        }

        let bytes = builder.into_inner()?;
        Ok(Set::from_bytes(bytes)?)
    }
}
