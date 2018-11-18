#![feature(custom_attribute)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod compiler;

fn main() {
    compiler::compile("./program/main.c");
}
