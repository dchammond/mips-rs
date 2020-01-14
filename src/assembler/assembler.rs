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

#[derive(Copy, Clone, PartialEq, Debug)]
enum MemRangeStatus {
    Free,
    Allocated,
}

impl Eq for MemRangeStatus {}

#[derive(Copy, Clone, Debug)]
struct MemRange {
    lower: u32,
    upper: u32, // inclusive
    status: MemRangeStatus,
}

#[derive(Copy, Clone, Debug)]
struct MemPosition {
    pub lower: Option<u32>,
    pub upper: Option<u32>, // inclusive
    pub size: u32,
}

impl MemPosition {
    pub fn new(lower: Option<u32>, upper: Option<u32>, size: u32) -> MemPosition {
        MemPosition { lower, upper, size }
    }
}

impl MemRange {
    pub fn new(lower: u32, upper: u32, status: MemRangeStatus) -> MemRange {
        if lower > upper {
            panic!("lower > upper");
        }
        MemRange {
            lower,
            upper,
            status,
        }
    }
    pub fn set_status(&mut self, status: MemRangeStatus) {
        self.status = status;
    }
    pub fn size_bytes(&self) -> u32 {
        self.upper - self.lower + 1
    }
    pub fn shrink(mut self, bytes: NonZeroU32) -> (MemRange, MemRange) {
        self.upper -= bytes.get();
        (
            self,
            MemRange::new(
                self.upper + 1,
                self.upper + 1 + bytes.get(),
                MemRangeStatus::Free,
            ),
        )
    }
    pub fn grow(
        mut self,
        next: Option<&[MemRange]>,
        bytes: NonZeroU32,
    ) -> (MemRange, Option<Vec<MemRange>>) {
        self.upper += bytes.get();
        let mut r = None;
        if let Some(ranges) = next {
            let mut tmp = ranges
                .into_iter()
                .cloned()
                .skip_while(|mem| {
                    if MemRangeStatus::Free != mem.status {
                        panic!("Tried to grow into non-free block: {:?} -> {:?}", self, mem);
                    }
                    mem.upper < self.upper
                })
                .collect::<Vec<MemRange>>();
            if tmp.len() > 0 {
                let next = tmp[0];
                tmp[0] = MemRange::new(self.upper + 1, next.upper, MemRangeStatus::Free);
            }
            r = Some(tmp);
        }
        (self, r)
    }
    /**
     * "Middle" will always be returned as the second element of the tuple
     * The first element exists if there is a MemRange lower than middle
     * The third element exists if there is a MemRange higher than middle
     */
    pub fn insert(self, middle: MemRange) -> (Option<MemRange>, MemRange, Option<MemRange>) {
        if MemRangeStatus::Free != self.status {
            panic!(
                "Tried to insert into non-free block: {:?} <-> {:?}",
                self, middle
            );
        }
        if middle.lower < self.lower || self.upper < middle.upper {
            panic!("Tried to insert out of bounds: {:?} <- {:?}", self, middle);
        }
        // TODO: clenaup and use match like in first_fit
        if middle.lower == self.lower && middle.upper == self.upper {
            (None, middle, None)
        } else if middle.lower == self.lower {
            (
                None,
                middle,
                Some(MemRange::new(
                    middle.upper + 1,
                    self.upper,
                    MemRangeStatus::Free,
                )),
            )
        } else if middle.upper == self.upper {
            (
                Some(MemRange::new(
                    self.lower,
                    middle.lower - 1,
                    MemRangeStatus::Free,
                )),
                middle,
                None,
            )
        } else {
            (
                Some(MemRange::new(
                    self.lower,
                    middle.lower - 1,
                    MemRangeStatus::Free,
                )),
                middle,
                Some(MemRange::new(
                    middle.upper + 1,
                    self.upper,
                    MemRangeStatus::Free,
                )),
            )
        }
    }
    pub fn merge(mut self, other: MemRange) -> MemRange {
        if self.upper + 1 != other.lower {
            panic!("Is there a hole? {:?} <-> {:?}", self, other);
        }
        if MemRangeStatus::Free != other.status {
            panic!(
                "Cannot merge with non-free memory block: {:?} <-> {:?}",
                self, other
            );
        }
        self.upper = other.upper;
        self
    }
    fn first_fit_insert(position: MemPosition, memory: &mut Vec<MemRange>) {
        let l = position.lower.unwrap();
        let u = position.upper.unwrap();
        if let Some((i, m)) = memory
            .iter()
            .enumerate()
            .find(|(_, mem)| mem.lower <= l && u <= mem.upper && MemRangeStatus::Free == mem.status)
        {
            let result = m.insert(MemRange::new(l, u, MemRangeStatus::Allocated));
            memory.remove(i);
            if let Some(x) = result.2 {
                memory.insert(i, x);
            }
            memory.insert(i, result.1);
            if let Some(x) = result.0 {
                memory.insert(i, x);
            }
        } else {
            panic!(
                "Could not find placement for: {:?} in {:?}",
                position, memory
            );
        }
    }
    fn first_fit_insert_arbitray(size: u32, memory: &mut Vec<MemRange>) {
        if let Some((i, m)) = memory
            .iter()
            .enumerate()
            .find(|(_, mem)| MemRangeStatus::Free == mem.status && size <= mem.size_bytes())
        {
            let result = m.insert(MemRange::new(
                m.lower,
                m.lower + size - 1,
                MemRangeStatus::Allocated,
            ));
            memory.remove(i);
            if let Some(x) = result.2 {
                memory.insert(i, x);
            }
            memory.insert(i, result.1);
            if let Some(x) = result.0 {
                memory.insert(i, x);
            }
        } else {
            panic!(
                "Could not find placement for: {} bytes in {:?}",
                size, memory
            );
        }
    }
    pub fn first_fit(positions: &[MemPosition], min: u32, max: u32) -> Vec<MemRange> {
        let mut memory = vec![MemRange::new(min, max, MemRangeStatus::Free)];
        let mut arbitraries: Vec<MemPosition> = Vec::new();
        positions.into_iter().for_each(|pos| match pos {
            MemPosition {
                lower: Some(l),
                upper: Some(u),
                size,
            } => {
                MemRange::first_fit_insert(*pos, &mut memory);
            }
            MemPosition {
                lower: Some(l),
                upper: None,
                size,
            } => {
                let u = l + size - 1;
                let pos = MemPosition::new(Some(*l), Some(u), *size);
                MemRange::first_fit_insert(pos, &mut memory);
            }
            MemPosition {
                lower: None,
                upper: Some(u),
                size,
            } => {
                let l = u + 1 - size;
                let pos = MemPosition::new(Some(l), Some(*u), *size);
                MemRange::first_fit_insert(pos, &mut memory);
            }
            MemPosition {
                lower: None,
                upper: None,
                size,
            } => {
                arbitraries.push(*pos);
            }
        });
        arbitraries.into_iter().for_each(|pos| {
            MemRange::first_fit_insert_arbitray(pos.size, &mut memory);
        });
        memory
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
}

fn assign_addresses(parsed: &mut Parsed, labels: &mut HashMap<String, NonZeroU32>) {
    layout_text_segment(&mut parsed.text_segment, labels);
}

pub fn assemble(mut parsed: Parsed) {
    let mut labels: HashMap<String, NonZeroU32> = HashMap::new();
    assign_addresses(&mut parsed, &mut labels);
    //expand_pseudo(&mut parsed.text_segment);
}
