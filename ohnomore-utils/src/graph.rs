use conllx::Token;
use failure::{err_msg, Error};
use petgraph::graph::Graph;
use petgraph::Directed;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyNode {
    pub token: Token,
    pub offset: usize,
}

pub type DependencyGraph = Graph<Token, String, Directed>;

pub fn sentence_to_graph(sentence: &[Token]) -> Result<DependencyGraph, Error> {
    let mut g = Graph::new();

    let mut nodes = Vec::new();
    for token in sentence.iter() {
        nodes.push(g.add_node(token.clone()));
    }

    for (idx, token) in sentence.iter().enumerate() {
        let rel = match token.head_rel() {
            Some(rel) => rel.to_owned(),
            None => return Err(err_msg(format!("Token without head relation: {}", token))),
        };

        if let Some(head) = token.head() {
            if head != 0 {
                g.add_edge(nodes[head - 1], nodes[idx], rel);
            }
        }
    }

    Ok(g)
}
