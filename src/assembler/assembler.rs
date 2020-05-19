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

/*
fn expand_pseudo(text_segments: &mut [TextSegment]) {
    let insertions: Vec<((usize, usize), (Inst, Inst))> = Vec::new();
    for (segment_idx, text_segment) in text_segments.iter().enumerate() {
        for (instr_idx, instruction) in text_segment.instructions.iter().enumerate() {
            match &instruction.1 {
                Inst::ILabel(ITypeLabel{opcode: IInst::la, rs, rt, label}) => {

                },
                _ => (),
            }
        }
    }
}
*/

pub type SymbolTable = HashMap<String, u32>;

pub fn assemble(mut parsed: Parsed) {
    let mut labels: SymbolTable = HashMap::new();
    resolver::assign_addresses(&mut parsed, &mut labels);
    //expand_pseudo(&mut parsed.text_segment);
}
