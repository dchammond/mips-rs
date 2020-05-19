use crate::{
    instructions::{itype::*, jtype::*, Inst},
    machine::{address::Address, memory::*},
    parser::parser::{
        DataBytes, DataCString, DataEntry, DataHalfs, DataSegment, DataSpace,
        DataWords, Parsed, TextSegment,
    },
    assembler::resolver,
};

use std::{collections::HashMap, convert::TryFrom, num::NonZeroU32};

pub struct Assembled {}

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
                        }) => {
                            let mut new_label_hi = label.label.unwrap().pop().unwrap();
                            let mut new_label_lo = new_label_hi.clone();
                            new_label_hi.push_str("@hi");
                            new_label_lo.push_str("@lo");
                            let lui = ITypeLabel::new(IInst::lui, rs, rt, Address::from(new_label_hi));
                            let ori = ITypeLabel::new(IInst::ori, rs, rt, Address::from(new_label_lo));
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
    //expand_pseudo(&mut parsed.text_segment);
}
