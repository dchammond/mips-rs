use crate::{
    instructions::{itype::*, jtype::*, Inst},
    machine::{address::Address, memory::*, register::*},
    parser::parser::{
        DataBytes, DataCString, DataEntry, DataHalfs, DataSegment, DataSpace,
        DataWords, Parsed, TextSegment,
    },
    assembler::resolver,
};

use std::{collections::HashMap, convert::TryFrom, num::NonZeroU32};

pub struct Assembled {}

fn expand_li_la_label(rs: Reg, rt: Reg, label: Address) -> (ITypeLabel, ITypeLabel) {
    let mut new_label_hi = label.label.unwrap().pop().unwrap();
    let mut new_label_lo = new_label_hi.clone();
    new_label_hi.push_str("@hi");
    new_label_lo.push_str("@lo");
    let lui = ITypeLabel::new(IInst::lui, rs, rt, Address::from(new_label_hi));
    let ori = ITypeLabel::new(IInst::ori, rs, rt, Address::from(new_label_lo));
    (lui, ori)
}

fn expand_li_la_imm(rs: Reg, rt: Reg, imm: u32) -> (ITypeImm, ITypeImm) {
    let lui = ITypeImm::new(IInst::lui, rs, rt, imm >> 16);
    let ori = ITypeImm::new(IInst::ori, rs, rt, imm & 0xFFFF);
    (lui, ori)
}

fn expand_pseudo(text_segments: Vec<TextSegment>) -> Vec<TextSegment> {
    let new_text_segments = Vec::with_capacity(text_segments.len());
    text_segments
        .into_iter()
        .for_each(|segment| {
            let mut new_segment = TextSegment::new();
            new_segment.start_address = segment.start_address;
            segment.instructions
                .into_iter()
                .for_each(|(addr, inst)| {
                    match inst {
                        Inst::ILabel(ITypeLabel {
                            opcode: IInst::la,
                            rs,
                            rt,
                            label
                        }) |
                        Inst::ILabel(ITypeLabel {
                            opcode: IInst::li,
                            rs,
                            rt,
                            label
                        }) => {
                            let (lui, ori) = expand_li_la_label(rs, rt, label);
                            new_segment.instructions.push((addr, lui.into()));
                            new_segment.instructions.push((None, ori.into()));
                        },
                        Inst::IImm(ITypeImm {
                            opcode: IInst::la,
                            rs,
                            rt,
                            imm
                        }) |
                        Inst::IImm(ITypeImm {
                            opcode: IInst::li,
                            rs,
                            rt,
                            imm
                        }) => {
                            let (lui, ori) = expand_li_la_imm(rs, rt, imm);
                            new_segment.instructions.push((addr, lui.into()));
                            new_segment.instructions.push((None, ori.into()));
                        },
                        //Inst::JLabel() => ,
                        _ => new_segment.instructions.push((addr, inst)),
                    }
                });
        });
    new_text_segments
}

pub type SymbolTable = HashMap<String, u32>;

pub fn assemble(mut parsed: Parsed) {
    let mut labels: SymbolTable = HashMap::new();
    resolver::assign_addresses(&mut parsed, &mut labels);
    println!("{:#?}", labels);
    //expand_pseudo(&mut parsed.text_segment);
}
