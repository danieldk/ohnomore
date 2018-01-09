#[macro_use]
extern crate error_chain;

extern crate fst;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

extern crate petgraph;

pub mod automaton;

pub mod constants;

pub mod error;

pub mod lookup;

extern crate serde;

#[macro_use]
extern crate serde_derive;

pub mod transform;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;
