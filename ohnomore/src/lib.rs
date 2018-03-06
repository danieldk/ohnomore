extern crate conllx;

extern crate fst;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

extern crate petgraph;

pub mod automaton;

pub mod constants;

pub mod error;

#[macro_use]
extern crate error_chain;

#[macro_use]
mod macros;

pub mod transform;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
