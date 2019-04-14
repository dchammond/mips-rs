use lazy_static::lazy_static;
use regex::Regex;

use std::vec::Vec;
use std::ops::{Index, IndexMut};
use std::str::*;

#[derive(Clone, Debug)]
pub struct Parsed {
    pub data_segment: Option<Segment>,
    pub text_segment: Option<Segment>,
    pub kdata_segment: Option<Segment>,
    pub ktext_segment: Option<Segment>,
}

impl Parsed {
    pub fn new(data_segment: Option<Segment>, text_segment: Option<Segment>, kdata_segment: Option<Segment>, ktext_segment: Option<Segment>) -> Parsed {
        Parsed {data_segment, text_segment, kdata_segment, ktext_segment}
    }
}

#[derive(Clone, Debug)]
pub struct SegmentEntry {
    offset: u32,           // offset from segment start
    label: Option<String>, // If the segment is labeled
    alignment: Alignment,  // alignment of each entry
    data: Vec<[u8; 4]>,    // size of an entry is data.len() * Alignment
                           // Accessing a data element is based off the alignment
}

pub trait ToFromBytes {
    fn to_bytes(&self) -> [u8; 4];
    fn from_bytes(bytes: [u8; 4]) -> Self;
}

impl ToFromBytes for u8 {
    fn to_bytes(&self) -> [u8; 4] {
        let mut out = [0u8; 4];
        out.clone_from_slice(&self.to_le_bytes());
        out
    }
    fn from_bytes(bytes: [u8; 4]) -> Self {
        bytes[0]
    }
}

impl ToFromBytes for u16 {
    fn to_bytes(&self) -> [u8; 4] {
        let mut out = [0u8; 4];
        out.clone_from_slice(&self.to_le_bytes());
        out
    }
    fn from_bytes(bytes: [u8; 4]) -> Self {
        Self::from_le_bytes([bytes[0], bytes[1]])
    }
}

impl ToFromBytes for u32 {
    fn to_bytes(&self) -> [u8; 4] {
        let mut out = [0u8; 4];
        out.clone_from_slice(&self.to_le_bytes());
        out
    }
    fn from_bytes(bytes: [u8; 4]) -> Self {
        Self::from_le_bytes(bytes)
    }
}

#[allow(dead_code)]
impl SegmentEntry {
    pub fn new<T,W,U,V>(offset: T, label: Option<W>, alignment: U, data: &[V]) -> SegmentEntry where u32: From<T>, String: From<W>, Alignment: From<U>, V: ToFromBytes {
        if data.len() == 0 {
            SegmentEntry {offset: offset.into(), label: label.map(|s| String::from(s)), alignment: alignment.into(), data: Vec::new()}
        } else {
            let mut v: Vec<[u8; 4]> = Vec::with_capacity(data.len());
            for d in data {
                v.push(d.to_bytes());
            }
            SegmentEntry {offset: offset.into(), label: label.map(|s| String::from(s)), alignment: alignment.into(), data: v}
        }
    }
    pub fn add_data<T>(&mut self, data: &T) where T: ToFromBytes {
        self.data.push(data.to_bytes());
    }
    pub fn get_data_checked(&self, idx: usize) -> Option<&[u8]> {
        if idx >= self.data.len() {
            None
        } else {
            Some(&self[idx])
        }
    }
    pub fn get_data_mut_checked(&mut self, idx: usize) -> Option<&mut [u8]> {
        if idx >= self.data.len() {
            None
        } else {
            Some(&mut self[idx])
        }
    }
}

impl Index<usize> for SegmentEntry {
    type Output = [u8];
    fn index(&self, idx: usize) -> &Self::Output {
        match self.alignment {
            Alignment::Byte     => &self.data[idx][..1],
            Alignment::HalfWord => &self.data[idx][..2],
            Alignment::Word     => &self.data[idx][..4],
        }
    }
}

impl IndexMut<usize> for SegmentEntry {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match self.alignment {
            Alignment::Byte     => &mut self.data[idx][..1],
            Alignment::HalfWord => &mut self.data[idx][..2],
            Alignment::Word     => &mut self.data[idx][..4],
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Alignment {
    Byte,
    HalfWord,
    Word,
}

macro_rules! alignment_map {
    ($type_name: ty) => (
        impl From<$type_name> for Alignment {
            fn from(n: $type_name) -> Alignment {
                match n {
                    1 => Alignment::Byte,
                    2 => Alignment::HalfWord,
                    4 => Alignment::Word,
                    _ => panic!("Invalid alignemnt: {}", n)
                }
            }
        }
    );
}

macro_rules! alignment_inv_map {
    ($type_name: ty) => (
        impl From<Alignment> for $type_name {
            fn from(a: Alignment) -> Self {
                match a {
                    Alignment::Byte => 1,
                    Alignment::HalfWord => 2,
                    Alignment::Word => 4,
                }
            }
        }
    );
}

alignment_map!(u8);
alignment_map!(u16);
alignment_map!(u32);
alignment_map!(u64);
alignment_map!(u128);
alignment_map!(usize);
alignment_inv_map!(u8);
alignment_inv_map!(u16);
alignment_inv_map!(u32);
alignment_inv_map!(u64);
alignment_inv_map!(u128);
alignment_inv_map!(usize);

#[derive(Clone, Debug)]
pub struct Segment {
    entries: Vec<SegmentEntry>
}

impl Segment {
    pub fn new(entries: Vec<SegmentEntry>) -> Segment {
        Segment {entries}
    }
    pub fn get_size(&self) -> u32 {
        let mut size: u32 = 0;
        self.entries.iter().for_each(|e| size += (e.data.len() * usize::from(e.alignment)) as u32);
        size
    }
    pub fn get_entries(&self) -> &[SegmentEntry] {
        &self.entries[..]
    }
}

#[derive(Copy, Clone, Debug)]
enum ParseMode {
    Default,
    Data,
    Text,
    KData,
    KText,
}

pub fn parse(program: &String) -> Parsed {
    lazy_static! {
        static ref LABEL_RE: Regex = Regex::new(r"^\s*(?P<label>\w+):\s*$").unwrap();
        static ref DIRECTIVE_RE: Regex = Regex::new(r"\s*\.(?P<directive>\w+)\s*").unwrap();
        static ref J_STR_RE: Regex = Regex::new(r"^\s*(?P<opcode>\w+)\s*(?P<label>\w+)s*$").unwrap();
        static ref R_ARITH_RE: Regex = Regex::new(r"^\s*(?P<funct>\w+)\s*(?P<rd>\$[\w\d]+),\s*(?P<rs>\$[\w\d]+),\s*(?P<rt>\$[\w\d]+)\s*$").unwrap();
        static ref R_SHIFT_HEX_RE: Regex = Regex::new(r"^\s*(?P<funct>\w+)\s*(?P<rd>\$[\w\d]+),\s*(?P<rs>\$[\w\d]+),\s*0x(?P<shamt>[\da-fA-F]+)\s*$").unwrap();
        static ref R_SHIFT_DEC_RE: Regex = Regex::new(r"^\s*(?P<funct>\w+)\s*(?P<rd>\$]\w\d]+),\s*(?P<rs>\$[\w\d]+),\s*(?P<shamt>\d+)\s*$").unwrap();
        static ref R_JUMP_RE: Regex = Regex::new(r"^\s*(?P<funct>\w+)\s*(?P<rs>\$[\w\d]+)\s*$").unwrap();
        static ref I_ARITH_HEX_RE:  Regex = Regex::new(r"^\s*(?P<opcode>\w+)\s*(?P<rt>\$[\w\d]+?),\s*(?P<rs>\$[\w\d]+?),\s*0x(?P<imm>[\da-fA-F]+)\s*$").unwrap();
        static ref I_ARITH_DEC_RE:  Regex = Regex::new(r"^\s*(?P<opcode>\w+)\s*(?P<rt>\$[\w\d]+?),\s*(?P<rs>\$[\w\d]+?),\s*(?P<imm>\d+)\s*$").unwrap();
        static ref I_BRANCH_HEX_RE: Regex = Regex::new(r"^\s*(?P<opcode>\w+)\s*(?P<rt>\$[\w\d]+?),\s*(?P<rs>\$[\w\d]+?),\s*0x(?P<imm>[\da-fA-F]+)\s*$").unwrap();
        static ref I_BRANCH_STR_RE: Regex = Regex::new(r"^\s*(?P<opcode>\w+)\s*(?P<rt>\$[\w\d]+?),\s*(?P<rs>\$[\w\d]+?),\s*(?P<label>\w+)\s*$").unwrap();
        static ref I_MEM_HEX_RE:    Regex = Regex::new(r"^\s*(?P<opcode>\w+)\s*(?P<rt>\$[\w\d]+?),\s*0x(?P<imm>[\da-fA-F]+)\((?P<rs>\$[\w\d]+?)\)\s*$").unwrap();
        static ref I_MEM_DEC_RE:    Regex = Regex::new(r"^\s*(?P<opcode>\w+)\s*(?P<rt>\$[\w\d]+?),\s*(?P<imm>\d+)\((?P<rs>\$[\w\d]+?)\)\s*$").unwrap();
        static ref I_MEM_STR_RE:    Regex = Regex::new(r"^\s*(?P<opcode>\w+)\s*(?P<rt>\$[\w\d]+?),\s*(?P<label>\w+)\((?P<rs>\$[\w\d]+?)\)\s*$").unwrap();
        static ref I_IMM_HEX_RE:  Regex = Regex::new(r"^\s*(?P<opcode>\w+)\s*(?P<rt>\$[\w\d]+?),\s*0x(?P<imm>[\da-fA-F]+)\s*$").unwrap();
        static ref I_IMM_DEC_RE:  Regex = Regex::new(r"^\s*(?P<opcode>\w+)\s*(?P<rt>\$[\w\d]+?),\s*(?P<imm>\d+)\s*$").unwrap();
        static ref I_IMM_STR_RE:  Regex = Regex::new(r"^\s*(?P<opcode>\w+)\s*(?P<rt>\$[\w\d]+?),\s*(?P<label>\w+)\s*$").unwrap();
    }
    let mut data_seg = None;
    let mut text_seg = None;
    let mut kdata_seg = None;
    let mut ktext_seg = None;
    let mut parse_mode = ParseMode::Default;
    let mut current_label: Option<String> = None;
    let mut lines: Lines = program.lines();
    while let Some(line) = lines.next() {

    }
    Parsed::new(data_seg, text_seg, kdata_seg, ktext_seg)
}

