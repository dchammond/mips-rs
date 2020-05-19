use crate::{
    instructions::{itype::*, jtype::*, Inst},
    machine::{address::Address, memory::*},
    parser::parser::{
        DataEntry,DataSegment, Parsed, TextSegment,
    },
    assembler::assembler::SymbolTable,
};

use std::{collections::HashMap, convert::TryFrom, num::NonZeroU32};


fn define_labels(a: &Address, addr: u32, labels: &mut SymbolTable) {
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
    labels: &mut SymbolTable,
    max_addr: u32
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
                define_labels(a, non_zero_addr.get(), labels);
            }
            addr += 4;
            if addr >= max_addr {
                panic!("Text segment too large");
            }
            inst.0 = Some(Address::from(non_zero_addr));
        });
}

fn generate_hi_lo_labels(address: &mut Address) {
    let mut splits: Vec<String> = Vec::new();
    address.label
        .as_ref()
        .unwrap()
        .iter()
        .for_each(|label| {
            let mut high = label.clone();
            high.push_str("@hi");
            let mut low  = label.clone();
            low.push_str("@lo");
            splits.push(high);
            splits.push(low);
        });
    address.label.as_mut().unwrap().append(&mut splits);
}

fn assign_data_segment_addresses(
    data_segment: &mut DataSegment,
    labels: &mut SymbolTable,
    max_addr: u32
) {
    let mut addr: u32 = data_segment
        .start_address
        .as_ref()
        .unwrap()
        .numeric
        .as_ref()
        .unwrap()
        .get();
    data_segment.
        data_entries
        .iter_mut()
        .for_each(|entry: &mut DataEntry| {
            let non_zero_addr = unsafe { NonZeroU32::new_unchecked(addr) };
            match entry {
                DataEntry::CString(ref mut c) => {
                    if let Some(ref mut a) = &mut c.chars.0 {
                        generate_hi_lo_labels(a);
                        define_labels(a, non_zero_addr.get(), labels);
                    }
                    addr += c.size() as u32;
                    if addr >= max_addr {
                        panic!("Data segment too large");
                    }
                    c.chars.0 = Some(Address::from(non_zero_addr));
                },
                DataEntry::Bytes(ref mut b) => {
                    if let Some(ref mut a) = &mut b.bytes.0 {
                        generate_hi_lo_labels(a);
                        define_labels(a, non_zero_addr.get(), labels);
                    }
                    addr += b.size() as u32;
                    if addr >= max_addr {
                        panic!("Data segment too large");
                    }
                    b.bytes.0 = Some(Address::from(non_zero_addr));
                },
                DataEntry::Halfs(ref mut h) => {
                    if let Some(ref mut a) = &mut h.halfs.0 {
                        generate_hi_lo_labels(a);
                        define_labels(a, non_zero_addr.get(), labels);
                    }
                    addr += h.size() as u32;
                    if addr >= max_addr {
                        panic!("Data segment too large");
                    }
                    h.halfs.0 = Some(Address::from(non_zero_addr));
                },
                DataEntry::Words(ref mut w) => {
                    if let Some(ref mut a) = &mut w.words.0 {
                        generate_hi_lo_labels(a);
                        define_labels(a, non_zero_addr.get(), labels);
                    }
                    addr += w.size() as u32;
                    if addr >= max_addr {
                        panic!("Data segment too large");
                    }
                    w.words.0 = Some(Address::from(non_zero_addr));
                },
                DataEntry::Space(ref mut s) => {
                    if let Some(ref mut a) = &mut s.spaces.0 {
                        generate_hi_lo_labels(a);
                        define_labels(a, non_zero_addr.get(), labels);
                    }
                    addr += s.size() as u32;
                    if addr >= max_addr {
                        panic!("Data segment too large");
                    }
                    s.spaces.0 = Some(Address::from(non_zero_addr));
                },
            }
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
    labels: &mut SymbolTable,
    min_addr: u32,
    max_addr: u32
) {
    let positions = text_segment_entries
        .iter()
        .map(|segment| {
            let size_bytes = segment.size() as u32;
            let mut lower = None;
            if let Some(start) = &segment.start_address {
                if let Some(numeric) = start.numeric {
                    lower = Some(numeric.get());
                }
            }
            MemPosition::new(lower, None, size_bytes, Some(segment))
        })
        .collect::<Vec<MemPosition<TextSegment>>>();
    let ranges = FirstFitAllocator::layout(&positions, min_addr, max_addr);
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
        assign_text_segment_addresses(t, labels, max_addr);
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
                            .unwrap();
                        let offset = calculate_offset::<u16>(*label_addr, inst_addr);
                        inst.1 = Inst::IImm(ITypeImm::new(
                            i_type_label.opcode,
                            i_type_label.rs,
                            i_type_label.rt,
                            offset,
                        ));
                    }
                    Inst::JLabel(j_type) => {
                        let label_addr = labels
                            .get(j_type.label.label.as_ref().unwrap().get(0).unwrap())
                            .unwrap();
                        let offset = calculate_offset::<u32>(*label_addr, inst_addr);
                        inst.1 = Inst::JImm(JTypeImm::new(j_type.opcode, offset));
                    }
                    _ => {}
                }
            });
    });
}

fn layout_data_segment(
    data_segment_entries: &mut [DataSegment],
    labels: &mut SymbolTable,
    min_addr: u32,
    max_addr: u32
) {
    let positions = data_segment_entries
        .iter()
        .map(|segment| {
            let size_bytes = segment.size() as u32;
            let mut lower = None;
            if let Some(start) = &segment.start_address {
                if let Some(numeric) = start.numeric {
                    lower = Some(numeric.get());
                }
            }
            MemPosition::new(lower, None, size_bytes, Some(segment))
        })
        .collect::<Vec<MemPosition<DataSegment>>>();
    let ranges = FirstFitAllocator::layout(&positions, min_addr, max_addr);
    let mut indexes: Vec<(usize, u32)> = Vec::with_capacity(data_segment_entries.len());
    ranges.into_iter().for_each(|range| {
        if let Some(data_ref) = range.get_data() {
            let found = data_segment_entries
                .iter()
                .enumerate()
                .find(|(_, t)| std::ptr::eq(*t, data_ref))
                .unwrap();
            indexes.push((found.0, range.get_range().0));
        }
    });
    indexes.into_iter().for_each(|(idx, lower)| {
        let lower = NonZeroU32::new(lower);
        if let Some(addr) = data_segment_entries[idx].start_address.as_mut() {
            addr.numeric = lower;
        } else {
            data_segment_entries[idx].start_address = Some(Address::new(lower, None));
        }
    });
    data_segment_entries.iter_mut().for_each(|t| {
        assign_data_segment_addresses(t, labels, max_addr);
    });
}

pub fn assign_addresses(parsed: &mut Parsed, labels: &mut SymbolTable) {
    // STATIC_DATA has no defined size, but we allocate greedily so
    // we should have no issues with using up all of the dynamic space
    layout_data_segment(&mut parsed.data_segment, labels, STATIC_DATA_START, STACK_START);
    layout_data_segment(&mut parsed.kdata_segment, labels, KERNEL_DATA_END, KERNEL_DATA_START);
    layout_text_segment(&mut parsed.text_segment, labels, TEXT_START, TEXT_END);
    layout_text_segment(&mut parsed.ktext_segment, labels, KERNEL_TEXT_END, KERNEL_TEXT_START);
}
