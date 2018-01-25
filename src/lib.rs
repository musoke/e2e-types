#![feature(try_from)]
#![feature(ascii_ctype)]
extern crate core;
#[macro_use]
extern crate failure;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate serde_json;

pub mod device;
