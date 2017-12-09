use petgraph::graph::{DiGraph, NodeIndex};

type DependencyGraph<T> = DiGraph<T, String>;

pub trait Token {
    fn form(&self) -> &str;
    fn lemma(&self) -> &str;
    fn tag(&self) -> &str;
}

pub trait Transform<T>
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String;
}

pub mod delemmatization;

pub mod lemmatization;

#[cfg(test)]
pub(crate) mod test_helpers;
