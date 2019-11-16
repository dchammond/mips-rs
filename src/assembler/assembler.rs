use crate::{
    instructions::{itype::*, jtype::*, rtype::*, Inst},
    machine::{address::Address, register::Reg},
    parser::parser::{
        DataAlignment, DataBytes, DataCString, DataEntry, DataHalfs, DataSegment, DataSpace,
        DataWords, KDataSegment, KTextSegment, Parsed, TextSegment,
    },
};

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

fn assign_text_segment_addresses(text_segment: &mut [TextSegment]) {
}

fn assign_addresses(parsed: &mut Parsed) {
    assign_text_segment_addresses(&mut parsed.text_segment);
}

pub fn assemble(mut parsed: Parsed) {
    assign_addresses(&mut parsed);
    //expand_pseudo(&mut parsed.text_segment);
}
