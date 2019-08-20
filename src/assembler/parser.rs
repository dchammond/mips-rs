use std::str::Lines;
use std::vec::Vec;
use std::num::ParseIntError;

use crate::assembler::parsing_functions::*;
use crate::instructions::itype::*;
use crate::instructions::rtype::*;
use crate::instructions::jtype::*;
use crate::instructions::Inst;
use crate::machine::register::Reg;
use crate::machine::immediate::Imm;

#[derive(Clone, Debug)]
pub struct Parsed {
    pub text_segment: Vec<Inst>,
}

impl Parsed {
    pub fn new() -> Parsed {
        Parsed {text_segment: Vec::new()}
    }
}

fn i_extract_imm(imm: (Option<&str>, Result<i64, ParseIntError>)) -> Option<i64> {
    let mut imm_int: i64 = match imm.1 {
        Ok(i) => i as i64,
        Err(_) => return None,
    };
    if let Some(sign) = imm.0 {
        if sign == "-" {
            imm_int *= -1;
        }
    };
    Some(imm_int)
}

fn parse_text_segment(parsed: &mut Parsed, lines: &mut Lines) -> Option<String> {
    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() || entire_line_is_comment(line) {
            return Some(line.to_owned());
            //continue;
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
            Ok((_, (inst, rt, rs, imm))) => {
                let imm_int = match i_extract_imm(imm) {
                    Some(i) => i,
                    None => panic!("Unable to parse immediate: {}", line),
                };
                parsed.text_segment.push(IType::new(IInst::from(inst), Reg::from(rs), Reg::from(rt), Imm::from(imm_int as u64)).into());
                continue;
            },
            Err(_) => (),
        }
        match i_branch_imm(line) {
            Ok((_, (inst, rt, rs, imm))) => {
                let imm_int = match i_extract_imm(imm) {
                    Some(i) => i,
                    None => panic!("Unable to parse immediate: {}", line),
                };
                parsed.text_segment.push(IType::new(IInst::from(inst), Reg::from(rs), Reg::from(rt), Imm::from(imm_int as u64)).into());
                continue;
            },
            Err(_) => (),
        }
        match i_branch_label(line) {
            Ok((_, (inst, rt, rs, label))) => {
                let _ = label; // TODO: convert label to number
                parsed.text_segment.push(IType::new(IInst::from(inst), Reg::from(rs), Reg::from(rt), Imm::from(0u64)).into());
                continue;
            },
            Err(_) => (),
        }
        match i_mem_imm(line) {
            Ok((_, (inst, rt, imm, rs))) => {
                let imm_int = match i_extract_imm(imm) {
                    Some(i) => i,
                    None => panic!("Unable to parse immediate: {}", line),
                };
                parsed.text_segment.push(IType::new(IInst::from(inst), Reg::from(rs), Reg::from(rt), Imm::from(imm_int as u64)).into());
                continue;
            },
            Err(_) => (),
        }
        match i_mem_label(line) {
            Ok((_, (inst, rt, label, rs))) => {
                let _ = label; // TODO: convert label to number
                parsed.text_segment.push(IType::new(IInst::from(inst), Reg::from(rs), Reg::from(rt), Imm::from(0u64)).into());
                continue;
            },
            Err(_) => (),
        }
        match i_load_imm(line) {
            Ok((_, (inst, rt, imm))) => {
                let imm_int = match i_extract_imm(imm) {
                    Some(i) => i,
                    None => panic!("Unable to parse immediate: {}", line),
                };
                parsed.text_segment.push(IType::new(IInst::from(inst), Reg::zero, Reg::from(rt), Imm::from(imm_int as u64)).into());
                continue;
            },
            Err(_) => (),
        }
        match i_load_label(line) {
            Ok((_, (inst, rt, label))) => {
                let _ = label; // TODO: convert label to number
                parsed.text_segment.push(IType::new(IInst::from(inst), Reg::zero, Reg::from(rt), Imm::from(0u64)).into());
                continue;
            },
            Err(_) => (),
        }
        match j_label(line) {
            Ok((_, (inst, label))) => {
                let _ = label; // TODO: convert label to number
                parsed.text_segment.push(JType::new(JInst::from(inst), Imm::from(0u64)).into());
                continue;
            },
            Err(_) => (),
        }
        panic!("Uknown line in text section: {}", line);
    }
    None
}

pub fn parse(program: &str) -> Parsed {
    let mut parsed = Parsed::new();
    let mut lines: Lines = program.lines();
    while let Some(line) = lines.next() {
        let mut line = line.trim();
        if line.is_empty() || entire_line_is_comment(line) {
            continue;
        }
        let ret: String;
        // pretend we got a .text directive
        match parse_text_segment(&mut parsed, &mut lines) {
            Some(l) => ret = l,
            None => continue,
        }
        line = ret.trim();
    }
    parsed
}
