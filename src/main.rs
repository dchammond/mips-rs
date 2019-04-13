use std::{env, path};

use mips_rs::*;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <mips_file>", args[0]);
        return;
    }
    let mut state = State::new();
    load_file(&mut state, path::Path::new(&args[1]), None);
    state.run();
    println!("registers:\n{:?}", state);
}

