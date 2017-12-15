extern crate conllx;
extern crate getopts;
extern crate ohnomore;
extern crate ohnomore_utils;
extern crate petgraph;
extern crate stdinout;

use std::env::args;

use getopts::Options;
use ohnomore::constants::{LEMMA_IS_FORM_TAGS, NO_LEMMA_TAGS, SPECIAL_LEMMA_TAGS};
use ohnomore::transform::{Token, Transform};
use ohnomore::transform::delemmatization::{RemoveAlternatives, RemoveAuxTag, RemovePassivTag, RemoveSepVerbPrefix};
use ohnomore_utils::graph::sentence_to_gold_graph;
use petgraph::graph::NodeIndex;
use stdinout::{Input, OrExit};

use ohnomore_utils::graph::{GoldDependencyGraph, GoldToken};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [CORPUS]", program);
    print!("{}", opts.usage(&brief));
}

fn apply_transformations<T>(g: &mut GoldDependencyGraph, idx: NodeIndex,
                            transformations: &[T]) where T: AsRef<Transform<GoldToken>> {
    for t in transformations {
        let lemma = t.as_ref().transform(g, idx);
        g[idx].set_lemma(lemma);
    }
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = opts.parse(&args[1..])
        .or_exit("Cannot parse command-line options", 1);

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.len() > 1 {
        print_usage(&program, opts);
        return;
    }

    let transforms: &[Box<Transform<GoldToken>>] = &[Box::new(RemoveAlternatives), Box::new(RemoveAuxTag), Box::new(RemovePassivTag), Box::new(RemoveSepVerbPrefix)];

    let input = Input::from(matches.free.get(0));
    let reader = conllx::Reader::new(input.buf_read().or_exit("Cannot read corpus", 1));

    for sentence in reader {
        let sentence = sentence.or_exit("Cannot read sentence", 1);
        let mut graph = sentence_to_gold_graph(&sentence).or_exit("Error constructing graph", 1);

        for node in graph.node_indices() {
            {
                let pos = graph[node].tag();
                if LEMMA_IS_FORM_TAGS.contains(pos) || NO_LEMMA_TAGS.contains(pos) || SPECIAL_LEMMA_TAGS.contains(pos) {
                    continue;
                }
            }

            apply_transformations(&mut graph, node, transforms);
            let token = &graph[node];
            println!("{}\t{}\t{}", token.form(), token.tag(), token.lemma());
        }
    }
}
