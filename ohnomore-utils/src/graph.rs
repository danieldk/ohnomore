use conllx::Token;
use failure::Error;
use petgraph::Directed;
use petgraph::graph::Graph;

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
        assert!(token.head_rel().is_some(), "missing dependency relation");
        let rel = token.head_rel().unwrap().to_owned();

        if let Some(head) = token.head() {
            if head != 0 {
                g.add_edge(nodes[head - 1], nodes[idx], rel);
            }
        }
    }

    Ok(g)
}
