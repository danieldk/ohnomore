extern crate caseless;

extern crate conllx;

extern crate failure;

extern crate fst;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

extern crate petgraph;

extern crate seqalign;

extern crate unicode_normalization;

pub mod automaton;

pub mod constants;

#[macro_use]
mod macros;

pub mod transform;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
