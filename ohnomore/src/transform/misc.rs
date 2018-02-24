//! Miscellaneous transformations.
//!
//! This module provides transformations that can be used for both
//! lemmatization and delemmatization.

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

    use super::{SimplifyArticleLemma, SimplifyPossesivePronounLemma};

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
}
