use conllx;

use petgraph::graph::{DiGraph, NodeIndex};

pub type DependencyGraph<T> = DiGraph<T, String>;

pub trait MutableToken {
    fn set_lemma(&mut self, lemma: Option<String>);
}

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
        self.lemma().unwrap_or("_")
    }

    fn tag(&self) -> &str {
        self.pos().unwrap()
    }
}

impl MutableToken for conllx::Token {
    fn set_lemma(&mut self, lemma: Option<String>) {
        self.set_lemma(lemma);
    }
}

pub trait Transform<T>
where
    T: Token,
{
    fn transform(&self, graph: &DependencyGraph<T>, node: NodeIndex) -> String;
}

/// A list of `Transform`s.
pub struct Transforms<T>(pub Vec<Box<dyn Transform<T>>>)
where
    T: Token + MutableToken;

impl<T> Transforms<T>
where
    T: Token + MutableToken,
{
    /// Transform a graph using the transformation list.
    ///
    /// This method applies the transformations to the given graph. Each
    /// transform is fully applied to the graph before the next transform,
    /// to ensure that dependencies between transforms are correctly handled.
    pub fn transform(&self, graph: &mut DependencyGraph<T>) {
        for t in &self.0 {
            for node in graph.node_indices() {
                let lemma = t.as_ref().transform(graph, node);
                graph[node].set_lemma(Some(lemma));
            }
        }
    }
}

pub mod delemmatization;

pub mod lemmatization;

pub mod misc;

mod named_entity;

mod svp;

#[cfg(test)]
pub(crate) mod test_helpers;
