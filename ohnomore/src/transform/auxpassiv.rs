use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::Direction;

use constants::*;

use transform::{DependencyGraph, Token};

pub enum VerbLemmaTag {
    Auxiliary,
    Passive,
    None,
}

pub fn verb_lemma_tag<T>(g: &DependencyGraph<T>, node: NodeIndex) -> VerbLemmaTag
where
    T: Token,
{
    let token = &g[node];
    let lemma = token.lemma();

    match g.edges_directed(node, Direction::Outgoing)
        .find(|e| e.weight() == AUXILIARY_RELATION)
    {
        Some(edge) => {
            if lemma == PASSIVE_VERB_LEMMA && g[edge.target()].tag() == PARTICIPLE_TAG {
                VerbLemmaTag::Passive
            } else {
                VerbLemmaTag::Auxiliary
            }
        }
        None => VerbLemmaTag::None,
    }
}

/// Return the ancestor following a path.
///
/// *Note:* this function assumes single-headedness.
pub fn ancestor_path<T, S>(
    g: &DependencyGraph<T>,
    mut node: NodeIndex,
    path: &[S],
) -> Option<NodeIndex>
where
    S: AsRef<str>,
{
    for rel in path {
        let edge = g.edges_directed(node, Direction::Incoming)
            .find(|e| e.weight() == rel.as_ref())?;
        node = edge.source();
    }

    Some(node)
}
