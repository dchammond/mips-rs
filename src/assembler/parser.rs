use std::str::Lines;
use std::vec::Vec;
use std::num::{ParseIntError, NonZeroU32};

use crate::assembler::parsing_functions::*;
use crate::instructions::itype::*;
use crate::instructions::rtype::*;
use crate::instructions::jtype::*;
use crate::instructions::Inst;
use crate::machine::register::Reg;
use crate::machine::immediate::Imm;

#[derive(Clone, Debug)]
pub struct Address {
    pub numeric: Option<NonZeroU32>,
    pub label: Option<String>,
}

impl Address {
    pub fn new(numeric: Option<NonZeroU32>, label: Option<String>) -> Address {
        Address {numeric, label}
    }
}

#[derive(Clone, Debug)]
pub struct TextSegment {
    pub instructions: Vec<(Option<Address>, Inst)>,
    pub start_address: Option<Address>,
}

impl TextSegment {
    pub fn new() -> TextSegment {
        TextSegment {instructions: Vec::new(), start_address: None}
    }

    pub fn push_instruction(&mut self, inst: Inst) {
        self.instructions.push((None, inst));
    }

    pub fn push_addressed_instruction(&mut self, addr: Address, inst: Inst) {
        self.instructions.push((Some(addr), inst));
    }
}

#[derive(Clone, Debug)]
pub struct Parsed {
    pub text_segment: Vec<TextSegment>,
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

fn parse_text_segment(lines: &mut Lines, text_segment: &mut TextSegment) -> Option<String> {
    for line in lines {
        let line = line.trim();
        if line.is_empty() || entire_line_is_comment(line) {
            return Some(line.to_owned());
        }
        // for now assume this line will not be directive
        if let Ok((_, (inst, rd, rs, rt))) = r_arithmetic(line) {
            text_segment.push_instruction(RType::new(RInst::from(inst), Reg::from(rs), Reg::from(rt), Reg::from(rd), 0).into());
            continue;
        }
        if let Ok((_, (inst, rd, rt, shamt))) = r_shift(line) {
            if let Some(sign) = shamt.0 {
                if sign == "-" {
                    panic!("Cannot have negative shift amount: {}", line);
                }
            }
            let shamt_int = match shamt.1 {
                Ok(i) => i as u8,
                Err(p) => panic!("Unable to parse shift amount: {} because {}", line, p),
            };
            text_segment.push_instruction(RType::new(RInst::from(inst), Reg::zero, Reg::from(rt), Reg::from(rd), shamt_int).into());
            continue;
        }
        if let Ok((_, (inst, rs))) = r_jump(line) {
            text_segment.push_instruction(RType::new(RInst::from(inst), Reg::from(rs), Reg::zero, Reg::zero, 0).into());
            continue;
        }
        if let Ok((_, (inst, rt, rs, imm))) = i_arith(line) {
            let imm_int = match i_extract_imm(imm) {
                Some(i) => i,
                None => panic!("Unable to parse immediate: {}", line),
            };
            text_segment.push_instruction(IType::new(IInst::from(inst), Reg::from(rs), Reg::from(rt), Imm::from(imm_int as u64)).into());
            continue;
        }
        if let Ok((_, (inst, rt, rs, imm))) = i_branch_imm(line) {
            let imm_int = match i_extract_imm(imm) {
                Some(i) => i,
                None => panic!("Unable to parse immediate: {}", line),
            };
            text_segment.push_instruction(IType::new(IInst::from(inst), Reg::from(rs), Reg::from(rt), Imm::from(imm_int as u64)).into());
            continue;
        }
        if let Ok((_, (inst, rt, rs, label))) = i_branch_label(line) {
            let _ = label; // TODO: convert label to number
            text_segment.push_instruction(IType::new(IInst::from(inst), Reg::from(rs), Reg::from(rt), Imm::from(0u64)).into());
            continue;
        }
        if let Ok((_, (inst, rt, imm, rs))) = i_mem_imm(line) {
            let imm_int = match i_extract_imm(imm) {
                Some(i) => i,
                None => panic!("Unable to parse immediate: {}", line),
            };
            text_segment.push_instruction(IType::new(IInst::from(inst), Reg::from(rs), Reg::from(rt), Imm::from(imm_int as u64)).into());
            continue;
        }
        if let Ok((_, (inst, rt, label, rs))) = i_mem_label(line) {
            let _ = label; // TODO: convert label to number
            text_segment.push_instruction(IType::new(IInst::from(inst), Reg::from(rs), Reg::from(rt), Imm::from(0u64)).into());
            continue;
        }
        if let Ok((_, (inst, rt, imm))) = i_load_imm(line) {
            let imm_int = match i_extract_imm(imm) {
                Some(i) => i,
                None => panic!("Unable to parse immediate: {}", line),
            };
            text_segment.push_instruction(IType::new(IInst::from(inst), Reg::zero, Reg::from(rt), Imm::from(imm_int as u64)).into());
            continue;
        }
        if let Ok((_, (inst, rt, label))) = i_load_label(line) {
            let _ = label; // TODO: convert label to number
            text_segment.push_instruction(IType::new(IInst::from(inst), Reg::zero, Reg::from(rt), Imm::from(0u64)).into());
            continue;
        }
        if let Ok((_, (inst, label))) = j_label(line) {
            let _ = label; // TODO: convert label to number
            text_segment.push_instruction(JType::new(JInst::from(inst), Imm::from(0u64)).into());
            continue;
        }
        panic!("Uknown line in text section: {}", line);
    }
    None
}

pub fn parse(program: &str) -> Parsed {
    let mut parsed = Parsed::new();
    let mut lines = program.lines();
    while let Some(line) = lines.next() {
        // restructure this loop. the common case is that a directive parse function will return a line for a new section
        // when a directive function does _not_ return a line, we are at the end of a well-formed program
        let mut line = line.trim();
        if line.is_empty() || entire_line_is_comment(line) {
            continue;
        }
        let ret: String;
        if let Ok((_, Some(imm))) = directive_text(line) {
            let mut text_segment = TextSegment::new();

            match i_extract_imm(imm) {
                Some(i) => text_segment.start_address = Some(Address::new(NonZeroU32::new(i as u32), None)),
                None => (),
            }

            match parse_text_segment(&mut lines, &mut text_segment) {
                Some(l) => ret = l,
                None => (),
            }
            
            if text_segment.instructions.len() > 0 {
                parsed.text_segment.push(text_segment);
            }
        } else {
            continue;
        }
        line = ret.trim();
    }
    parsed
}
