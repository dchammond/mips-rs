use crate::{
    instructions::{itype::*, jtype::*, Inst},
    machine::{address::Address, memory::*},
    parser::parser::{
        DataAlignment, DataBytes, DataCString, DataEntry, DataHalfs, DataSegment, DataSpace,
        DataWords, KDataSegment, KTextSegment, Parsed, TextSegment,
    },
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

fn define_labels(a: &Address, addr: NonZeroU32, labels: &mut HashMap<String, NonZeroU32>) {
    if let Some(v) = &a.label {
        v.iter().for_each(|s| {
            if labels.contains_key(s) {
                panic!(format!("Redefinition of label: {}", s));
            }
            labels.insert(s.clone(), addr);
        });
    } else {
        panic!("Expected non empty labels in Address");
    }
}

fn assign_text_segment_addresses(
    text_segment: &mut TextSegment,
    labels: &mut HashMap<String, NonZeroU32>,
) {
    let mut addr: u32 = text_segment
        .start_address
        .as_ref()
        .unwrap()
        .numeric
        .as_ref()
        .unwrap()
        .get();
    text_segment
        .instructions
        .iter_mut()
        .for_each(|inst: &mut (Option<Address>, Inst)| {
            let non_zero_addr = unsafe { NonZeroU32::new_unchecked(addr) };
            if let Some(a) = &inst.0 {
                define_labels(a, non_zero_addr, labels);
            }
            addr += 4;
            if addr >= TEXT_END {
                panic!("Text segment too large");
            }
            inst.0 = Some(Address::from(non_zero_addr));
        });
}

fn calculate_offset<T>(label_addr: u32, inst_addr: u32) -> T
where T: TryFrom<u32> + std::ops::Not<Output = T> + std::ops::Add<Output = T>,
     <T as TryFrom<u32>>::Error: std::fmt::Debug
{
    if label_addr > inst_addr {
        T::try_from((label_addr - inst_addr) >> 2).expect(&format!(
                "instruction and label too far apart: {:#X} <-> {:#X}",
                inst_addr >> 2,
                label_addr >> 2
        ))
    } else {
        let pos =
            T::try_from((inst_addr - label_addr) >> 2).expect(&format!(
                    "instruction and label too far apart: {:#X} <-> {:#X}",
                    inst_addr >> 2,
                    label_addr >> 2
            ));
        !pos + T::try_from(1u32).unwrap()
    }
}

// Just a first-come-first-served-first-fit allocator
// Two passes 1. handle Segments with a desired address
// 2. find a place for everything else
// All TextSegments should have a defined start address
// before passed too assign_text_segment_addresses
fn layout_text_segment(
    text_segment_entries: &mut [TextSegment],
    labels: &mut HashMap<String, NonZeroU32>,
) {
    let positions = text_segment_entries
        .iter()
        .map(|segment| {
            let size_bytes = (segment.instructions.len() * 4) as u32;
            let mut lower = None;
            if let Some(start) = &segment.start_address {
                if let Some(numeric) = start.numeric {
                    lower = Some(numeric.get());
                }
            }
            MemPosition::new(lower, None, size_bytes, Some(segment))
        })
        .collect::<Vec<MemPosition<TextSegment>>>();
    let ranges = FirstFitAllocator::layout(&positions, TEXT_START, TEXT_END);
    let mut indexes: Vec<(usize, u32)> = Vec::with_capacity(text_segment_entries.len());
    ranges.into_iter().for_each(|range| {
        if let Some(data_ref) = range.get_data() {
            let found = text_segment_entries
                .iter()
                .enumerate()
                .find(|(_, t)| std::ptr::eq(*t, data_ref))
                .unwrap();
            indexes.push((found.0, range.get_range().0));
        }
    });
    indexes.into_iter().for_each(|(idx, lower)| {
        let lower = NonZeroU32::new(lower);
        if let Some(addr) = text_segment_entries[idx].start_address.as_mut() {
            addr.numeric = lower;
        } else {
            text_segment_entries[idx].start_address = Some(Address::new(lower, None));
        }
    });
    text_segment_entries.iter_mut().for_each(|t| {
        assign_text_segment_addresses(t, labels);
    });
    text_segment_entries.iter_mut().for_each(|t| {
        t.instructions
            .iter_mut()
            .for_each(|inst: &mut (Option<Address>, Inst)| {
                let inst_addr: u32 = inst.0.as_ref().unwrap().numeric.unwrap().get();
                match &inst.1 {
                    Inst::ILabel(i_type_label) => {
                        let label_addr = labels
                            .get(i_type_label.label.label.as_ref().unwrap().get(0).unwrap())
                            .unwrap()
                            .get();
                        let offset: u16 = calculate_offset(label_addr, inst_addr);
                        inst.1 = Inst::IImm(ITypeImm::new(
                            i_type_label.opcode,
                            i_type_label.rs,
                            i_type_label.rt,
                            offset,
                        ));
                    },
                    Inst::JLabel(j_type) => {
                        let label_addr = labels
                            .get(j_type.label.label.as_ref().unwrap().get(0).unwrap())
                            .unwrap()
                            .get();
                        let offset: u32 = calculate_offset(label_addr, inst_addr);
                        inst.1 = Inst::JImm(JTypeImm::new(j_type.opcode, offset));
                    }
                    _ => {}
                }
            });
    });
}

fn assign_addresses(parsed: &mut Parsed, labels: &mut HashMap<String, NonZeroU32>) {
    layout_text_segment(&mut parsed.text_segment, labels);
}

pub fn assemble(mut parsed: Parsed) {
    let mut labels: HashMap<String, NonZeroU32> = HashMap::new();
    assign_addresses(&mut parsed, &mut labels);
    println!("{:#?}", labels);
    println!("{:#?}", parsed);
    //expand_pseudo(&mut parsed.text_segment);
}
