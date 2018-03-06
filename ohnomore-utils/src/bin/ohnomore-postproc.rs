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
use ohnomore::transform::Transforms;
use ohnomore::transform::lemmatization::{AddAuxPassivTag, AddSeparatedVerbPrefix, FormAsLemma,
                                         MarkVerbPrefix, ReadVerbPrefixes, RestoreCase};
use ohnomore::transform::misc::{SimplifyArticleLemma, SimplifyPossesivePronounLemma};
use ohnomore_utils::graph::sentence_to_graph;
use stdinout::{Input, OrExit, Output};

fn print_usage(program: &str, opts: Options) {
    let brief = format!(
        "Usage: {} [options] VERB_PREFIXES [INPUT] [OUTPUT]",
        program
    );
    print!("{}", opts.usage(&brief));
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

    let transforms = Transforms(vec![
        Box::new(FormAsLemma),
        Box::new(RestoreCase),
        Box::new(AddSeparatedVerbPrefix::new(true)),
        Box::new(prefix_transform),
        Box::new(AddAuxPassivTag),
        Box::new(SimplifyArticleLemma),
        Box::new(SimplifyPossesivePronounLemma),
    ]);

    let input = Input::from(matches.free.get(1));
    let reader = conllx::Reader::new(input.buf_read().or_exit("Cannot read corpus", 1));

    let output = Output::from(matches.free.get(2));
    let mut writer = conllx::Writer::new(BufWriter::new(
        output.write().or_exit("Cannot open file for writing", 1),
    ));

    for sentence in reader {
        let sentence = sentence.or_exit("Cannot read sentence", 1);
        let mut graph = sentence_to_graph(&sentence).or_exit("Error constructing graph", 1);

        transforms.transform(&mut graph);

        let preproc_sentence: Vec<_> = graph.node_indices().map(|idx| graph[idx].clone()).collect();
        writer
            .write_sentence(preproc_sentence)
            .or_exit("Cannot write sentence", 1);
    }
}
