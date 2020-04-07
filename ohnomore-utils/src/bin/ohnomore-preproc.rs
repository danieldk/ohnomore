use std::env::args;
use std::io::BufWriter;

use conllx::io::WriteSentence;
use getopts::Options;
use ohnomore::transform::delemmatization::{
    RemoveAlternatives, RemoveReflexiveTag, RemoveSepVerbPrefix, RemoveTruncMarker,
};
use ohnomore::transform::lemmatization::FormAsLemma;
use ohnomore::transform::misc::{SimplifyArticleLemma, SimplifyPossesivePronounLemma};
use ohnomore::transform::Transforms;
use stdinout::{Input, OrExit, Output};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] [INPUT] [OUTPUT]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    let matches = opts
        .parse(&args[1..])
        .or_exit("Cannot parse command-line options", 1);

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.free.len() > 2 {
        print_usage(&program, opts);
        return;
    }

    let transforms = Transforms(vec![
        Box::new(RemoveAlternatives),
        Box::new(RemoveReflexiveTag),
        Box::new(RemoveSepVerbPrefix),
        Box::new(RemoveTruncMarker),
        Box::new(SimplifyArticleLemma),
        Box::new(SimplifyPossesivePronounLemma),
        Box::new(FormAsLemma),
    ]);

    let input = Input::from(matches.free.get(0));
    let reader = conllx::io::Reader::new(input.buf_read().or_exit("Cannot read corpus", 1));

    let output = Output::from(matches.free.get(1));
    let mut writer = conllx::io::Writer::new(BufWriter::new(
        output.write().or_exit("Cannot open file for writing", 1),
    ));

    for sentence in reader {
        let mut sentence = sentence.or_exit("Cannot read sentence", 1);

        transforms.transform(&mut sentence);

        writer
            .write_sentence(&sentence)
            .or_exit("Cannot write sentence", 1);
    }
}
