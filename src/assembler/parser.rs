use std::str::Lines;
use std::num::ParseIntError;

use crate::assembler::parsing_functions::*;
/*
#[derive(Clone, Debug)]
pub struct Parsed {
    pub text_segment:  Option<Segment>,
}

impl Parsed {
    pub fn new(data_segment:  Option<Segment>,
               text_segment:  Option<Segment>,
               kdata_segment: Option<Segment>,
               ktext_segment: Option<Segment>) -> Parsed {
        Parsed {data_segment, text_segment, kdata_segment, ktext_segment}
    }

    pub fn default() -> Parsed {
        Parsed {
            data_segment:  None,
            text_segment:  None,
            kdata_segment: None,
            ktext_segment: None,
        }
    }
}
*/

fn parse_text_segment(/*parsed: &mut Parsed,*/ lines: &mut Lines) {
    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() || entire_line_is_comment(line) {
            continue;
        }
        // for now assume this line will not be directive
        match r_arithmetic(line) {
            Ok((_, (inst, rd, rs, rt))) => continue,
            Err(_) => (),
        }
        match r_shift(line) {
            Ok((_, (inst, rd, rs, shamt))) => continue,
            Err(_) => (),
        }
        match r_jump(line) {
            Ok((_, (inst, rs))) => continue,
            Err(_) => (),
        }
        match i_arith(line) {
            Ok((_, (inst, rt, rs, imm))) => continue,
            Err(_) => (),
        }
        match i_branch_imm(line) {
            Ok((_, (inst, rt, rs, imm))) => continue,
            Err(_) => (),
        }
        match i_branch_label(line) {
            Ok((_, (inst, rt, rs, label))) => continue,
            Err(_) => (),
        }
        match i_mem_imm(line) {
            Ok((_, (inst, rt, imm, rs))) => continue,
            Err(_) => (),
        }
        match i_mem_label(line) {
            Ok((_, (inst, rt, label, rs))) => continue,
            Err(_) => (),
        }
        match i_load_imm(line) {
            Ok((_, (inst, rt, imm))) => continue,
            Err(_) => (),
        }
        match i_load_label(line) {
            Ok((_, (inst, rt, label))) => continue,
            Err(_) => (),
        }
        panic!("Uknown line in text section: {}", line);
    }
}

pub fn parse(program: &str) -> /*Parsed*/() {
    /*
    let mut parsed = Parsed::default();
    let mut lines: Lines = program.lines();
    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() || entire_line_is_comment(line) {
            continue;
        }
        // pretend we got a .text directive
        parse_text_segment(&mut parsed, &mut lines);
    }
    parsed
    */
}
