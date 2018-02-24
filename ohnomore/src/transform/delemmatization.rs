//! Delemmatization transformations.
//!
//! This module provides transformations that converts TüBa-D/Z-style lemmas
//! to `regular' lemmas.

use petgraph::graph::NodeIndex;

use constants::*;
use transform::{DependencyGraph, Token, Transform};

/// Remove alternative lemma analyses.
///
/// TüBa-D/Z sometimes provides multiple lemma analyses for a form. This
/// transformation removes all but the first analysis.
pub struct RemoveAlternatives;

impl<T> Transform<T> for RemoveAlternatives
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String {
        let token = &graph[node];
        let mut lemma = token.lemma();

        if token.tag().starts_with(PUNCTUATION_PREFIX) || token.tag() == NON_WORD_TAG
            || token.tag() == FOREIGN_WORD_TAG
        {
            return lemma.to_owned();
        }

        if let Some(idx) = lemma.find('|') {
            lemma = &lemma[..idx];
        }

        lemma.to_owned()
    }
}

/// Remove auxiliary markers.
///
/// The TüBa-D/Z marks auxiliary readings of applicable verbs using the *%aux*
/// suffix (e.g. *haben%aux*). This transformation rule removes such suffixes.
pub struct RemoveAuxTag;

impl<T> Transform<T> for RemoveAuxTag
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String {
        let token = &graph[node];
        let lemma = token.lemma();

        if token.tag().starts_with(AUXILIARY_PREFIX) || token.tag().starts_with(MODAL_PREFIX) {
            match lemma.rfind(AUXILIARY_MARKER) {
                Some(idx) => lemma[..idx].to_owned(),
                None => lemma.to_owned(),
            }
        } else {
            lemma.to_owned()
        }
    }
}

/// Remove passive markers.
///
/// The TüBa-D/Z marks passive readings of applicable verbs using the *%passiv*
/// suffixe (e.g. *werden%passiv*). This transformation rule removes such
/// suffixes.
pub struct RemovePassivTag;

impl<T> Transform<T> for RemovePassivTag
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String {
        let token = &graph[node];
        let lemma = token.lemma();

        if token.tag().starts_with(AUXILIARY_PREFIX) {
            match lemma.rfind(PASSIVE_MARKER) {
                Some(idx) => lemma[..idx].to_owned(),
                None => lemma.to_owned(),
            }
        } else {
            lemma.to_owned()
        }
    }
}

/// Replace reflexive tag.
///
/// Reflexives use the special *#refl* lemma in TüBa-D/Z. This transformation
/// replaces this pseudo-lemma by the lowercased form.
pub struct RemoveReflexiveTag;

impl<T> Transform<T> for RemoveReflexiveTag
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String {
        let token = &graph[node];
        let lemma = token.lemma();

        if token.tag() == REFLEXIVE_PERSONAL_PRONOUN_TAG
            && lemma == REFLEXIVE_PERSONAL_PRONOUN_LEMMA
        {
            return token.form().to_lowercase();
        }

        lemma.to_owned()
    }
}

/// Remove separable prefixes from verbs.
///
/// TüBa-D/Z marks separable verb prefixes in the verb lemma. E.g. *ab#zeichnen*,
/// where *ab* is the separable prefix. This transformation handles removes
/// separable prefixes from verbs. For example *ab#zeichnen* is transformed to
/// *zeichnen*.
pub struct RemoveSepVerbPrefix;

impl<T> Transform<T> for RemoveSepVerbPrefix
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String {
        let token = &graph[node];
        let mut lemma = token.lemma();

        if is_verb(token.tag()) {
            if let Some(idx) = lemma.rfind('#') {
                lemma = &lemma[idx + 1..];
            }
        }

        lemma.to_owned()
    }
}

/// Remove truncation markers.
///
/// TüBa-D/Z uses special marking for truncations. For example, *Bau-* in
///
/// *Bau- und Verkehrsplanungen*
///
/// is lemmatized as *Bauplanung%n*, recovering the full lemma and adding
/// a simplified part of speech tag of the word (since the form is tagged
/// as *TRUNC*).
///
/// This transformation replaces the TüBa-D/Z lemma by the word form, such
/// as *Bau-* in this example. If the simplified part of speech tag is not
/// *n*, the lemma is also lowercased.
pub struct RemoveTruncMarker;

impl<T> Transform<T> for RemoveTruncMarker
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String {
        let token = &graph[node];
        let lemma = token.lemma();

        if token.tag() == TRUNCATED_TAG {
            if let Some(idx) = lemma.rfind('%') {
                let tag = &lemma[idx + 1..];

                let form = if tag == "n" {
                    token.form().to_owned()
                } else {
                    token.form().to_lowercase()
                };

                return form;
            }
        }

        return lemma.to_owned();
    }
}

#[cfg(test)]
mod tests {
    use transform::test_helpers::run_test_cases;

    use super::{RemoveAuxTag, RemovePassivTag, RemoveSepVerbPrefix, RemoveTruncMarker};

    #[test]
    pub fn remove_auxiliary_tag() {
        run_test_cases("testdata/remove-aux-tag.test", RemoveAuxTag);
    }

    #[test]
    pub fn remove_passive_tag() {
        run_test_cases("testdata/remove-passive-tag.test", RemovePassivTag);
    }

    #[test]
    pub fn remove_sep_verb_prefix() {
        run_test_cases("testdata/remove-sep-verb-prefix.test", RemoveSepVerbPrefix);
    }

    #[test]
    pub fn remove_trunc_marker() {
        run_test_cases("testdata/remove-trunc-marker.test", RemoveTruncMarker);
    }
}
