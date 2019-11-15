use std::{env, path};

use mips_rs::parser::parser::*;
use mips_rs::*;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <mips_file>", args[0]);
        return;
    }
    let s = load_file(path::Path::new(&args[1]));
    println!("{:#?}", parse(&s));
}
