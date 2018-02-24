//! Miscellaneous transformations.
//!
//! This module provides transformations that can be used for both
//! lemmatization and delemmatization.

use std::collections::{HashMap, HashSet};

use petgraph::graph::NodeIndex;

use automaton::PrefixAutomaton;
use constants::*;
use fst::{IntoStreamer, Set, Streamer};
use transform::{DependencyGraph, Token, Transform};

/// Simplify article and relative pronoun lemmas.
///
/// This transformation simplifies lemmas of articles and relative pronouns
/// to *d* for definite and *e* for indefinite. For example:
///
/// * *den* -> *d*
/// * *einem* -> *e*
/// * *dessen* -> *d*
pub struct SimplifyArticleLemma;

impl<T> Transform<T> for SimplifyArticleLemma
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String {
        let token = &graph[node];
        let lemma = token.lemma();
        let form = token.form();
        let tag = token.tag();

        if tag == ARTICLE_TAG || tag == SUBST_REL_PRONOUN || tag == ATTR_REL_PRONOUN {
            if form.to_lowercase().starts_with("d") {
                return String::from("d");
            } else if form.to_lowercase().starts_with("e") {
                return String::from("e");
            }
        }

        return lemma.to_owned();
    }
}

lazy_static!{
    static ref PRONOUN_SIMPLIFICATIONS: HashMap<&'static str, HashSet<&'static str>> = hashmap! {
        "ich" => hashset!{"ich", "mich", "mir", "meiner"},
        "du" => hashset!{"du", "dir", "dich", "deiner"},
        "er" => hashset!{"er", "ihn", "ihm", "seiner"},
        "sie" => hashset!{"sie", "ihr", "ihnen", "ihrer"},
        "es" => hashset!{"es", "'s"},
        "wir" => hashset!{"wir", "uns", "unser"},
        "ihr" => hashset!{"euch"} // "ihr"
    };

    static ref PRONOUN_SIMPLIFICATIONS_LOOKUP: HashMap<String, String> =
        inside_out(&PRONOUN_SIMPLIFICATIONS);
}

fn inside_out(map: &HashMap<&'static str, HashSet<&'static str>>) -> HashMap<String, String> {
    let mut new_map = HashMap::new();

    for (&k, values) in map.iter() {
        for &value in values {
            new_map.insert(value.to_owned(), k.to_owned());
        }
    }

    new_map
}

/// Simplify personal pronouns.
///
/// This transformation simplifies personal pronouns using a simple lookup
/// of the lowercased word form. Pronouns are simplified with the following
/// rules (provided by Kathrin Beck):
///
/// Lowercased forms         | Lemma
/// -------------------------|------
/// *ich, mich, mir, meiner* | *ich*
/// *du, dir, dich, deiner*  | *du*
/// *er, ihn, ihm, seiner*   | *er*
/// *sie, ihr, ihnen, ihrer* | *sie*
/// *es, 's*                 | *es*
/// *wir, uns, unser*        | *wir*
/// *ihr, euch*              | *ihr*
///
/// In the case of the ambigious *ihr*, the lemma *sie* is always used.
pub struct SimplifyPersonalPronounLemma;

impl<T> Transform<T> for SimplifyPersonalPronounLemma
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String {
        let token = &graph[node];
        let tag = token.tag();
        let lemma = token.lemma();

        if tag != PERSONAL_PRONOUN_TAG {
            return lemma.to_owned();
        }

        let form = token.form().to_lowercase();
        if let Some(simplified_lemma) = PRONOUN_SIMPLIFICATIONS_LOOKUP.get(&form) {
            simplified_lemma.to_owned()
        } else {
            lemma.to_owned()
        }
    }
}

lazy_static! {
    static ref ATTR_POSS_PRONOUN_PREFIXES: Set = Set::from_iter(vec!["dein", "euer", "eure", "ihr", "mein", "sein", "unser"]).unwrap();

    static ref SUBST_POSS_PRONOUN_PREFIXES: Set = Set::from_iter(vec!["dein", "ihr", "mein", "sein", "unser", "unsrig"]).unwrap();
}

/// Simplify possesive pronoun lemmas.
///
/// This transformation simplifies pronoun lemmas to lemmas without
/// gender-specific suffixes. For example:
///
/// * *deinen* -> *dein*
/// * *deiner* -> *dein*
pub struct SimplifyPossesivePronounLemma;

impl<T> Transform<T> for SimplifyPossesivePronounLemma
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String {
        let token = &graph[node];
        let tag = token.tag();
        let form = token.form();
        let lemma = token.lemma();

        if tag != ATTRIBUTIVE_POSSESIVE_PRONOUN_TAG && tag != SUBST_POSSESIVE_PRONOUN_TAG {
            return lemma.to_owned();
        }

        let form = form.to_lowercase();
        let automaton = PrefixAutomaton::from(form.as_ref());
        let mut stream = if tag == SUBST_POSSESIVE_PRONOUN_TAG {
            SUBST_POSS_PRONOUN_PREFIXES.search(&automaton).into_stream()
        } else {
            ATTR_POSS_PRONOUN_PREFIXES.search(&automaton).into_stream()
        };

        if let Some(prefix) = stream.next() {
            let mut prefix = String::from_utf8(prefix.to_owned())
                .expect("Cannot decode prefix, PrefixAutomaton returned invalid prefix");
            if prefix == "eure" {
                prefix = "euer".to_owned();
            }

            return prefix.to_owned();
        }

        lemma.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use transform::test_helpers::run_test_cases;

    use super::{SimplifyArticleLemma, SimplifyPersonalPronounLemma, SimplifyPossesivePronounLemma};

    #[test]
    pub fn simplify_article_lemma() {
        run_test_cases("testdata/simplify-article-lemma.test", SimplifyArticleLemma);
    }

    #[test]
    pub fn simplify_possesive_pronoun_lemma() {
        run_test_cases(
            "testdata/simplify-possesive-pronoun-lemma.test",
            SimplifyPossesivePronounLemma,
        );
    }

    #[test]
    pub fn simplify_personal_pronoun_lemma() {
        run_test_cases("testdata/simplify-personal-pronoun.test", SimplifyPersonalPronounLemma);
    }
}
