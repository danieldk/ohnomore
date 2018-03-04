extern crate conllx;
extern crate getopts;
extern crate ohnomore;
extern crate ohnomore_utils;
extern crate petgraph;
extern crate stdinout;

use std::env::args;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use conllx::WriteSentence;
use getopts::Options;
use ohnomore::constants::{LEMMA_IS_FORM_TAGS, NO_LEMMA_TAGS};
use ohnomore::transform::{Token, Transform};
use ohnomore::transform::lemmatization::{AddAuxPassivTag, AddSeparatedVerbPrefix, MarkVerbPrefix,
                                         ReadVerbPrefixes};
use ohnomore::transform::misc::{SimplifyArticleLemma, SimplifyPossesivePronounLemma};
use ohnomore_utils::graph::sentence_to_graph;
use petgraph::graph::NodeIndex;
use stdinout::{Input, OrExit, Output};

use ohnomore_utils::graph::DependencyGraph;

fn print_usage(program: &str, opts: Options) {
    let brief = format!(
        "Usage: {} [options] VERB_PREFIXES [INPUT] [OUTPUT]",
        program
    );
    print!("{}", opts.usage(&brief));
}

fn apply_transformations<T>(g: &mut DependencyGraph, idx: NodeIndex, transformations: &[T])
where
    T: AsRef<Transform<conllx::Token>>,
{
    for t in transformations {
        let lemma = t.as_ref().transform(g, idx);
        g[idx].set_lemma(Some(lemma));
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

    if matches.free.is_empty() || matches.free.len() > 3 {
        print_usage(&program, opts);
        return;
    }

    let prefix_reader =
        BufReader::new(File::open(&matches.free[0]).or_exit("Cannot open verb prefix file", 1));
    let prefix_transform =
        MarkVerbPrefix::read_verb_prefixes(prefix_reader).or_exit("Cannot read verb prefixes", 1);

    let transforms: &[Box<Transform<conllx::Token>>] = &[
        Box::new(AddSeparatedVerbPrefix::new(true)),
        Box::new(prefix_transform),
        Box::new(AddAuxPassivTag),
        Box::new(SimplifyArticleLemma),
        Box::new(SimplifyPossesivePronounLemma),
    ];

    let input = Input::from(matches.free.get(1));
    let reader = conllx::Reader::new(input.buf_read().or_exit("Cannot read corpus", 1));

    let output = Output::from(matches.free.get(2));
    let mut writer = conllx::Writer::new(BufWriter::new(
        output.write().or_exit("Cannot open file for writing", 1),
    ));

    for sentence in reader {
        let sentence = sentence.or_exit("Cannot read sentence", 1);
        let mut graph = sentence_to_graph(&sentence).or_exit("Error constructing graph", 1);

        for node in graph.node_indices() {
            {
                let pos = graph[node].tag().to_owned();
                let form = graph[node].form().to_owned();
                if LEMMA_IS_FORM_TAGS.contains(pos.as_str()) {
                    graph[node].set_lemma(Some(form.to_lowercase()));
                }
            }
        }

        for node in graph.node_indices() {
            {
                let pos = graph[node].tag();
                if LEMMA_IS_FORM_TAGS.contains(pos) || NO_LEMMA_TAGS.contains(pos) {
                    continue;
                }
            }

            apply_transformations(&mut graph, node, transforms);
        }

        let preproc_sentence: Vec<_> = graph.node_indices().map(|idx| graph[idx].clone()).collect();
        writer
            .write_sentence(preproc_sentence)
            .or_exit("Cannot write sentence", 1);
    }
}
