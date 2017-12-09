use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

use constants::*;

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

#[cfg(test)]
mod tests {
    use transform::test_helpers::run_test_cases;

    use super::AddAuxPassivTag;

    #[test]
    pub fn add_aux_passiv_tag() {
        run_test_cases("testdata/add-aux-passiv-tag.test", AddAuxPassivTag);
    }
}
