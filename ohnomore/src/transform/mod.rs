use conllx;

use petgraph::graph::{DiGraph, NodeIndex};

pub type DependencyGraph<T> = DiGraph<T, String>;

pub trait Token {
    fn form(&self) -> &str;
    fn lemma(&self) -> &str;
    fn tag(&self) -> &str;
}

impl Token for conllx::Token {
    fn form(&self) -> &str {
        self.form()
    }

    fn lemma(&self) -> &str {
        self.lemma().unwrap()
    }

    fn tag(&self) -> &str {
        self.pos().unwrap()
    }
}

pub trait Transform<T>
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String;
}

mod auxpassiv;

pub mod delemmatization;

pub mod lemmatization;

pub mod misc;

mod svp;

#[cfg(test)]
pub(crate) mod test_helpers;
