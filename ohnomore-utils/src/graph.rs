use conllx;
use conllx::Sentence;
use ohnomore::transform::Token;
use petgraph::Directed;
use petgraph::graph::Graph;

use error::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyNode {
    pub token: conllx::Token,
    pub offset: usize,
}

pub struct GoldToken {
    form: String,
    tag: String,
    lemma: String,
}

impl GoldToken {
    pub fn set_lemma<L>(&mut self, lemma: L) where L: Into<String> {
        self.lemma = lemma.into()
    }
}

impl Token for GoldToken {
    fn form(&self) -> &str {
        &self.form
    }

    fn tag(&self) -> &str {
        &self.tag
    }

    fn lemma(&self) -> &str {
        &self.lemma
    }
}

pub type GoldDependencyGraph = Graph<GoldToken, String, Directed>;

pub fn sentence_to_gold_graph(sentence: &Sentence) -> Result<GoldDependencyGraph> {
    let mut g = Graph::new();

    let mut nodes = Vec::new();
    for token in sentence {
        let token = GoldToken {
            form: token.form().to_owned(),
            tag: token.pos().ok_or(ErrorKind::MissingTagLayer(format!("{}", token)))?.to_owned(),
            lemma: token.lemma().unwrap_or("_").to_owned()
        };

        nodes.push(g.add_node(token));
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
