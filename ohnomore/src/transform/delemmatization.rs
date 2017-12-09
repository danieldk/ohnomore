use petgraph::graph::NodeIndex;

use constants::*;
use transform::{DependencyGraph, Token, Transform};

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

#[cfg(test)]
mod tests {
    use transform::test_helpers::run_test_cases;

    use super::{RemoveAuxTag, RemovePassivTag};

    #[test]
    pub fn remove_auxiliary_tag() {
        run_test_cases("testdata/remove-aux-tag.test", RemoveAuxTag);
    }

    #[test]
    pub fn remove_passive_tag() {
        run_test_cases("testdata/remove-passive-tag.test", RemovePassivTag);
    }

}
