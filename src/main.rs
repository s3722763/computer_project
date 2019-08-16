#![feature(custom_attribute)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod compiler;

use std::fs;
use std::io::prelude::*;

fn main() {
    compiler::compile("./program/main.c");

/*    let mut file = fs::File::create("./program/program.asm").unwrap();
    file.write_all(&assembly_code.as_bytes());
    drop(file);*/
}
