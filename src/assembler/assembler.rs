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

// grows down
const TOP_RESERVED_SIZE: u32 = 0x0000_FFEF;
const TOP_RESERVED_START: u32 = 0xFFFF_FFFF;
const TOP_RESERVED_END: u32 = TOP_RESERVED_START - TOP_RESERVED_SIZE;

const MMIO_SIZE: u32 = 0x0000_0010;
const MMIO_START: u32 = TOP_RESERVED_END;
const MMIO_END: u32 = MMIO_START - MMIO_SIZE;

const KERNEL_DATA_SIZE: u32 = 0x6FFF_0000;
const KERNEL_DATA_START: u32 = MMIO_END;
const KERNEL_DATA_END: u32 = KERNEL_DATA_START - KERNEL_DATA_SIZE;

const KERNEL_TEXT_SIZE: u32 = 0x1000_0000;
const KERNEL_TEXT_START: u32 = KERNEL_DATA_END;
const KERNEL_TEXT_END: u32 = KERNEL_TEXT_START - KERNEL_TEXT_SIZE;

const STACK_START: u32 = KERNEL_TEXT_END;
// grows up
const STATIC_DATA_START: u32 = 0x1000_0000;

const TEXT_SIZE: u32 = 0x0600_0000;
const TEXT_END: u32 = STATIC_DATA_START;
const TEXT_START: u32 = TEXT_END - TEXT_SIZE;

const BOTTOM_RESERVED_SIZE: u32 = 0x0400_0000;
const BOTTOM_RESERVED_END: u32 = TEXT_START;
const BOTTOM_RESERVED_START: u32 = BOTTOM_RESERVED_END - BOTTOM_RESERVED_SIZE;

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

fn assign_text_segment_addresses(
    mut text_segment: TextSegment,
    labels: &mut HashMap<String, NonZeroU32>,
) -> TextSegment {
    let mut addr: u32 = text_segment
        .start_address
        .clone()
        .unwrap()
        .numeric
        .unwrap()
        .get();
    text_segment.instructions = text_segment
        .instructions
        .into_iter()
        .map(|inst: (Option<Address>, Inst)| {
            let non_zero_addr = unsafe { NonZeroU32::new_unchecked(addr) };
            if let Some(a) = inst.0 {
                match a.label {
                    Some(v) => {
                        v.into_iter().for_each(|s| {
                            if labels.contains_key(&s) {
                                panic!(format!("Redefinition of label: {}", s));
                            }
                            labels.insert(s, non_zero_addr);
                        });
                    }
                    None => panic!("Expected non empty labels in Address"),
                }
            }
            addr += 4;
            if addr >= TEXT_END {
                panic!("Text segment too large");
            }
            (Some(Address::from(non_zero_addr)), inst.1)
        })
        .collect();
    text_segment
}

}

fn assign_addresses(parsed: &mut Parsed, labels: &mut HashMap<String, NonZeroU32>) {
    layout_text_segment(&mut parsed.text_segment, labels);
}

pub fn assemble(mut parsed: Parsed) {
    let mut labels: HashMap<String, NonZeroU32> = HashMap::new();
    assign_addresses(&mut parsed, &mut labels);
    //expand_pseudo(&mut parsed.text_segment);
}
