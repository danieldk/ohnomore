use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use petgraph::graph::NodeIndex;

use transform::{DependencyGraph, Token, Transform};

pub struct TestToken {
    form: String,
    lemma: String,
    tag: String,
}

pub struct TestCase {
    graph: DependencyGraph<TestToken>,
    index: NodeIndex,
    correct: String,
}

impl Token for TestToken {
    fn form(&self) -> &str {
        &self.form
    }

    fn lemma(&self) -> &str {
        &self.lemma
    }

    fn tag(&self) -> &str {
        &self.tag
    }
}

fn read_dependency(iter: &mut Iterator<Item = &str>) -> Option<(String, TestToken)> {
    // If there is a relation, read it, otherwise bail out.
    let rel = iter.next()?.to_owned();

    // However, if there is a relation and no token, panic.
    Some((
        rel,
        read_token(iter).expect("Incomplete dependency relation"),
    ))
}

fn read_token(iter: &mut Iterator<Item = &str>) -> Option<TestToken> {
    Some(TestToken {
        form: iter.next()?.to_owned(),
        lemma: iter.next()?.to_owned(),
        tag: iter.next()?.to_owned(),
    })
}

fn read_test_cases<R>(buf_read: R) -> Vec<TestCase>
where
    R: BufRead,
{
    let mut test_cases = Vec::new();

    for line in buf_read.lines() {
        let line = line.unwrap();
        let line_str = line.trim();

        // Skip empty lines
        if line_str.is_empty() {
            continue;
        }

        // Skip comments
        if line_str.starts_with('#') {
            continue;
        }

        let mut iter = line.split_whitespace();

        let mut graph = DependencyGraph::new();

        let test_token = read_token(&mut iter).unwrap();
        let index = graph.add_node(test_token);
        let correct = iter.next().expect("Gold standard lemma missing").to_owned();

        // Optional: read head
        if let Some((rel, head)) = read_dependency(&mut iter) {
            let head_index = graph.add_node(head);
            graph.add_edge(head_index, index, rel);
        }

        // Optional: read dependents
        while let Some((rel, dep)) = read_dependency(&mut iter) {
            let dep_index = graph.add_node(dep);
            graph.add_edge(index, dep_index, rel);
        }

        let test_case = TestCase {
            graph,
            index,
            correct,
        };

        test_cases.push(test_case);
    }

    test_cases
}

pub fn run_test_cases<P, T>(filename: P, transform: T)
where
    P: AsRef<Path>,
    T: Transform<TestToken>,
{
    let f = File::open(filename).unwrap();
    let test_cases = read_test_cases(BufReader::new(f));

    for test_case in test_cases {
        assert_eq!(
            test_case.correct,
            transform.transform(&test_case.graph, test_case.index)
        )
    }
}
