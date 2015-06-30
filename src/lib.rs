#![feature(custom_derive, iter_arith, plugin, step_by, test)]
#![cfg_attr(test, deny(missing_docs, warnings))]
#![plugin(json_macros, num_macros, regex_macros)]

extern crate byteorder;
extern crate flate2;
extern crate nbt;
extern crate num;
extern crate regex;
extern crate rustc_serialize;
extern crate test;
extern crate time;
extern crate uuid;

pub mod packet;
pub mod proto;
pub mod types;
mod util;
