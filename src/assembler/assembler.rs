use crate::{
    instructions::{itype::*, jtype::*, rtype::*, Inst},
    machine::{address::Address, register::Reg},
    parser::parser::{
        DataAlignment, DataBytes, DataCString, DataEntry, DataHalfs, DataSegment, DataSpace,
        DataWords, KDataSegment, KTextSegment, Parsed, TextSegment,
    },
};

use std::{collections::HashMap, num::NonZeroU32};

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

fn assign_text_segment_addresses(text_segment: &mut TextSegment) {}

fn layout_text_segment(text_segment_entries: &mut [TextSegment], labels: &mut HashMap<String, NonZeroU32>) {

}

fn assign_addresses(parsed: &mut Parsed, labels: &mut HashMap<String, NonZeroU32>) {
    layout_text_segment(&mut parsed.text_segment, &mut labels);
}

pub fn assemble(mut parsed: Parsed) {
    let mut labels: HashMap<String, NonZeroU32> = HashMap::new();
    assign_addresses(&mut parsed, &mut labels);
    //expand_pseudo(&mut parsed.text_segment);
}
