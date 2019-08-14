use std::str::Lines;
use std::vec::Vec;
use std::num::ParseIntError;

use crate::assembler::parsing_functions::*;
use crate::instructions::rtype::*;
use crate::instructions::Inst;
use crate::machine::register::Reg;

#[derive(Clone, Debug)]
pub struct Parsed {
    pub text_segment: Vec<Inst>,
}

impl Parsed {
    pub fn new() -> Parsed {
        Parsed {text_segment: Vec::new()}
    }
}

fn parse_text_segment(parsed: &mut Parsed, lines: &mut Lines) {
    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() || entire_line_is_comment(line) {
            continue;
        }
        // for now assume this line will not be directive
        match r_arithmetic(line) {
            Ok((_, (inst, rd, rs, rt))) => {
                parsed.text_segment.push(RType::new(RInst::from(inst), Reg::from(rs), Reg::from(rt), Reg::from(rd), 0).into());
                continue;
            },
            Err(_) => (),
        }
        match r_shift(line) {
            Ok((_, (inst, rd, rt, shamt))) => {
                if let Some(sign) = shamt.0 {
                    if sign == "-" {
                        panic!("Cannot have negative shift amount: {}", line);
                    }
                }
                let shamt_int: u8 = match shamt.1 {
                    Ok(i) => i as u8,
                    Err(_) => panic!("Unable to parse shift amount: {}", line),
                };
                parsed.text_segment.push(RType::new(RInst::from(inst), Reg::zero, Reg::from(rt), Reg::from(rd), shamt_int).into());
                continue;
            },
            Err(_) => (),
        }
        match r_jump(line) {
            Ok((_, (inst, rs))) => {
                parsed.text_segment.push(RType::new(RInst::from(inst), Reg::from(rs), Reg::zero, Reg::zero, 0).into());
                continue;
            },
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

pub fn parse(program: &str) -> Parsed {
    let mut parsed = Parsed::new();
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
}
