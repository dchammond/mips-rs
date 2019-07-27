use lazy_static::lazy_static;
use regex::Regex;

use std::vec::Vec;
use std::ops::{Index, IndexMut};
use std::str::*;

#[derive(Clone, Debug)]
pub struct Parsed {
    pub data_segment: Option<Vec<Segment>>,
    pub text_segment: Option<Vec<Segment>>,
    pub kdata_segment: Option<Vec<Segment>>,
    pub ktext_segment: Option<Vec<Segment>>,
}

impl Parsed {
    pub fn new(data_segment: Option<Vec<Segment>>, text_segment: Option<Vec<Segment>>, kdata_segment: Option<Vec<Segment>>, ktext_segment: Option<Vec<Segment>>) -> Parsed {
        Parsed {data_segment, text_segment, kdata_segment, ktext_segment}
    }
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

#[derive(Clone, Debug)]
pub struct SegmentEntry {
    offset: u32,           // offset from segment start
    label: Option<String>, // If the segment is labeled
    alignment: Alignment,  // alignment of each entry
    data: Vec<[u8; 4]>,    // size of an entry is data.len() * Alignment
                           // Accessing a data element is based off the alignment
}

#[allow(dead_code)]
impl SegmentEntry {
    pub fn new<T,W,U>(offset: Option<T>, label: Option<W>, alignment: Option<U>) -> SegmentEntry where u32: From<T>, String: From<W>, Alignment: From<U> {
        SegmentEntry {offset: offset.map_or(0u32, |o| o.into()), label: label.map(|s| String::from(s)), alignment: alignment.map_or(Alignment::Byte, |a| a.into()), data: Vec::new()}
    }
    pub fn set_offset<T>(&mut self, offset: T) where u32: From<T> {
        self.offset = offset.into();
    }
    pub fn set_label<T>(&mut self, label: Option<T>) where String: From<T> {
        self.label = label.map(|l| l.into());
    }
    pub fn set_alignment<T>(&mut self, alignment: T) where Alignment: From<T> {
        self.alignment = alignment.into();
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
    pub fn get_alignment(&self) -> Alignment {
        self.alignment
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

alignment_map!(i8);
alignment_map!(i16);
alignment_map!(i32);
alignment_map!(i64);
alignment_map!(i128);
alignment_map!(isize);
alignment_map!(u8);
alignment_map!(u16);
alignment_map!(u32);
alignment_map!(u64);
alignment_map!(u128);
alignment_map!(usize);
alignment_inv_map!(i8);
alignment_inv_map!(i16);
alignment_inv_map!(i32);
alignment_inv_map!(i64);
alignment_inv_map!(i128);
alignment_inv_map!(isize);
alignment_inv_map!(u8);
alignment_inv_map!(u16);
alignment_inv_map!(u32);
alignment_inv_map!(u64);
alignment_inv_map!(u128);
alignment_inv_map!(usize);

#[derive(Clone, Debug)]
pub struct Segment {
    requested_start: Option<u32>, // user wants segment to start here
    entries: Vec<SegmentEntry>,
}

impl Segment {
    pub fn new<T>(start: Option<T>, entries: Option<Vec<SegmentEntry>>) -> Segment where u32: From<T> {
        Segment {requested_start: start.map(|s| s.into()), entries: entries.map_or(Vec::new(), |e| e)}
    }
    pub fn get_size(&self) -> u32 {
        let mut size: u32 = 0;
        self.entries.iter().for_each(|e| size += (e.data.len() * usize::from(e.alignment)) as u32);
        size
    }
    pub fn get_entries(&self) -> &[SegmentEntry] {
        &self.entries[..]
    }
    pub fn set_start<T>(&mut self, start: Option<T>) where u32: From<T> {
        self.requested_start = start.map(|s| s.into());
    }
    pub fn add_entry(&mut self, e: SegmentEntry) {
        self.entries.push(e);
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum ParseMode {
    Default,
    Data,
    Text,
    KData,
    KText,
}

fn match_number(s: &str) -> Option<i128> {
    lazy_static! {
        static ref NUM_HEX_RE: Regex = Regex::new(r"(?P<sign>[-+])?0x(?P<num>\d+)").unwrap();
        static ref NUM_DEC_RE: Regex = Regex::new(r"(?P<sign>[-+])?(?P<num>\d+)").unwrap();
    }
    if NUM_HEX_RE.is_match(s) {
        for caps in NUM_HEX_RE.captures_iter(s) {
            let mut i = i128::from_str_radix(&caps["addr"], 16).unwrap();
            if let Some(s) = caps.name("sign") {
                if s.as_str() == "-" {
                    i *= -1;
                }
            }
            return Some(i);
        }
    } else if NUM_DEC_RE.is_match(s) {
        for caps in NUM_DEC_RE.captures_iter(s) {
            let mut i = i128::from_str_radix(&caps["addr"], 16).unwrap();
            if let Some(s) = caps.name("sign") {
                if s.as_str() == "-" {
                    i *= -1;
                }
            }
            return Some(i);
        }
    }
    None
}

pub fn parse(program: &String) -> Parsed {
    lazy_static! {
        static ref LINE_COMMENT_RE: Regex = Regex::new(r"^(?P<comment>#*)$").unwrap();
        //static ref POST_COMMENT_RE: Regex = Regex::new(r"(?P<comment>#*)$").unwrap();
        static ref LABEL_RE: Regex = Regex::new(r"^(?P<label>\w[\w\d_]+):").unwrap();
        static ref DIRECTIVE_RE: Regex = Regex::new(r"\.(?P<directive>\w+\s*)").unwrap();
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
    let mut data_seg_vec: Option<Vec<Segment>> = None;
    let mut text_seg_vec: Option<Vec<Segment>> = None;
    let mut kdata_seg_vec: Option<Vec<Segment>> = None;
    let mut ktext_seg_vec: Option<Vec<Segment>> = None;
    let mut current_segment = Segment::new::<u32>(None, None);
    let mut current_segment_entry = SegmentEntry::new::<u32, String, Alignment>(None, None, None);
    let mut parse_mode = ParseMode::Default;
    let mut current_label: Option<String> = None;
    let mut lines: Lines = program.lines();
'parse_loop:
    while let Some(line) = lines.next() {
        let line = line.trim();
        if line == "" || LINE_COMMENT_RE.is_match(line) { // empty or pure comment line
            continue;
        }
        match parse_mode {
            ParseMode::Default => {
                for caps in DIRECTIVE_RE.captures_iter(line) {
                    match caps["directive"].trim() {
                        "data"  => parse_mode = ParseMode::Data,
                        "text"  => parse_mode = ParseMode::Text,
                        "kdata" => parse_mode = ParseMode::KData,
                        "ktext" => parse_mode = ParseMode::KText,
                        s => panic!("Expected segment directive, got: .{}", s)
                    }
                    if let Some(i) = match_number(line) {
                        current_segment.set_start(Some(i as u32));
                    }
                    break;
                }
                if parse_mode == ParseMode::Default {
                    panic!("Found code without any defined directive");
                }
            },
            ParseMode::Data | ParseMode::KData => {
                for caps in DIRECTIVE_RE.captures_iter(line) {
                    match caps["directive"].trim() {
                        "align" => {
                            if let Some(i) = match_number(line) {
                                current_segment.add_entry(current_segment_entry);
                                current_segment_entry = SegmentEntry::new::<u32, String, Alignment>(None, None, Some(i.into()));
                            }
                            continue 'parse_loop;
                        },
                        "data" | "kdata" => {
                            // If no starting address, just absorb into current data segment
                            if let Some(i) = match_number(line) {
                                current_segment.add_entry(current_segment_entry);
                                if parse_mode == ParseMode::Data {
                                    match data_seg_vec { Some(ref mut d) => d.push(current_segment), None => data_seg_vec = Some(vec![current_segment]), };
                                } else {
                                    match kdata_seg_vec { Some(ref mut d) => d.push(current_segment), None => kdata_seg_vec = Some(vec![current_segment]), };
                                }
                                current_segment = Segment::new::<u32>(Some(i as u32), None);
                                current_segment_entry = SegmentEntry::new::<u32, String, Alignment>(None, None, None);
                            }
                            continue 'parse_loop;
                        },
                        "text" => {
                            if parse_mode == ParseMode::Data {
                                match data_seg_vec { Some(ref mut d) => d.push(current_segment), None => data_seg_vec = Some(vec![current_segment]), };
                            } else {
                                match kdata_seg_vec { Some(ref mut d) => d.push(current_segment), None => kdata_seg_vec = Some(vec![current_segment]), };
                            }
                            current_segment = Segment::new::<u32>(Some(match_number(line).map_or(0u32, |i| i as u32)), None);
                            current_segment_entry = SegmentEntry::new::<u32, String, Alignment>(None, None, Some(Alignment::Word));
                        },
                        "space" => {
                            if let Some(i) = match_number(line) {
                                if i < 0 || i == 0 {
                                    panic!("Cannot allocate negative or zero space: {}", i);
                                }
                                let align = i128::from(current_segment_entry.get_alignment());
                                if i % align != 0 {
                                    panic!("Requested space size is not a multiple of current alignment: {} % {} != 0", i, align);
                                }
                                let mut i = i;
                                let zero = 0u32;
                                match current_segment_entry.get_alignment() {
                                    Alignment::Byte     => while i > 0 { current_segment_entry.add_data(&zero); i -= 1; },
                                    Alignment::HalfWord => while i > 0 { current_segment_entry.add_data(&zero); i -= 2; },
                                    Alignment::Word     => while i > 0 { current_segment_entry.add_data(&zero); i -= 4; },
                                }
                                current_segment.add_entry(current_segment_entry);
                                current_segment_entry = SegmentEntry::new::<u32, String, Alignment>(None, None, None);
                                continue 'parse_loop;
                            }
                            panic!("Found space directive without associated number");
                        },
                        "byte" => {
                            if let Some(i) = match_number(line) {

                            }
                        },
                        _ => { unimplemented!(); }
                    }
                    break;
                }
            }
            _ => unimplemented!()
        }
    }
    Parsed::new(data_seg_vec, text_seg_vec, kdata_seg_vec, ktext_seg_vec)
}
