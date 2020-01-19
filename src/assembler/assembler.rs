use crate::{
    instructions::{itype::*, jtype::*, rtype::*, Inst},
    machine::{address::Address, register::Reg, memory::*},
    parser::parser::{
        DataAlignment, DataBytes, DataCString, DataEntry, DataHalfs, DataSegment, DataSpace,
        DataWords, KDataSegment, KTextSegment, Parsed, TextSegment,
    },
};

use std::{collections::HashMap, convert::TryFrom, fmt::Debug, num::NonZeroU32};

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

fn define_labels(a: Address, addr: NonZeroU32, labels: &mut HashMap<String, NonZeroU32>) {
    if let Some(v) = a.label {
        v.into_iter().for_each(|s| {
            if labels.contains_key(&s) {
                panic!(format!("Redefinition of label: {}", s));
            }
            labels.insert(s, addr);
        });
    } else {
        panic!("Expected non empty labels in Address");
    }
}

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
                define_labels(a, non_zero_addr, labels);
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
    let ranges = MemRange::first_fit(&positions, TEXT_START, TEXT_END);
    let mut indexes: Vec<(usize, u32)> = Vec::with_capacity(text_segment_entries.len());
    ranges.into_iter().for_each(|range| {
        let data_ref = range.get_data().unwrap();
        let found = text_segment_entries
            .iter()
            .enumerate()
            .find(|(_, t)| std::ptr::eq(*t, data_ref))
            .unwrap();
        indexes.push((found.0, range.get_range().0));
    });
    indexes.into_iter().for_each(|(idx, lower)| {
        text_segment_entries[idx]
            .start_address
            .as_mut()
            .unwrap()
            .numeric
            .replace(NonZeroU32::new(lower).unwrap());
    });
    text_segment_entries.iter_mut().for_each(|t| {
        *t = assign_text_segment_addresses(t.clone(), labels);
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
                            .clone();
                        let label_addr: u32 = label_addr.get();
                        let offset = if label_addr > inst_addr {
                            u16::try_from((label_addr - inst_addr) >> 2).expect(&format!(
                                "instruction and label too far apart: {:#X} <-> {:#X}",
                                inst_addr >> 2,
                                label_addr >> 2
                            ))
                        } else {
                            let pos =
                                u16::try_from((inst_addr - label_addr) >> 2).expect(&format!(
                                    "instruction and label too far apart: {:#X} <-> {:#X}",
                                    inst_addr >> 2,
                                    label_addr >> 2
                                ));
                            !pos + 1
                        };
                        inst.1 = Inst::IImm(ITypeImm::new(
                            i_type_label.opcode,
                            i_type_label.rs,
                            i_type_label.rt,
                            offset,
                        ));
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
    //expand_pseudo(&mut parsed.text_segment);
}
