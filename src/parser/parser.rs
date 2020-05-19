use nom::Err;

use std::{
    iter::FromIterator,
    num::{NonZeroU32, ParseIntError},
    str::Lines,
    vec::Vec,
};

use crate::{
    instructions::{itype::*, jtype::*, rtype::*, Inst},
    machine::{address::Address, register::Reg},
    parser::parsing_functions::*,
};

#[derive(Clone, Debug)]
pub struct TextSegment {
    pub instructions: Vec<(Option<Address>, Inst)>,
    pub start_address: Option<Address>,
}

impl TextSegment {
    pub fn new() -> TextSegment {
        TextSegment {
            instructions: Vec::new(),
            start_address: None,
        }
    }

    pub fn size(&self) -> usize {
        self.instructions.len() * 4
    }
}

#[derive(Clone, Debug)]
pub struct DataCString {
    pub chars: (Option<Address>, Vec<u8>),
    pub alignment: NonZeroU32,
    pub null_terminated: bool,
}

impl DataCString {
    pub fn size(&self) -> usize {
        // a string is a unit so the whole thing is aligned
        let len = self.chars.1.len();
        let alignment = self.alignment.get() as usize;
        let padding = if alignment > len {
            alignment - len
        } else {
            len % alignment
        };
        len + padding
    }
}

#[derive(Clone, Debug)]
pub struct DataBytes {
    pub bytes: (Option<Address>, Vec<u8>),
    pub alignment: NonZeroU32,
}

impl DataBytes {
    pub fn size(&self) -> usize {
        // each byte is aligned
        let len = self.bytes.1.len();
        let padding = self.alignment.get() as usize - 1;
        let unit_size = padding + 1;
        len * unit_size
    }
}

#[derive(Clone, Debug)]
pub struct DataHalfs {
    pub halfs: (Option<Address>, Vec<u16>),
    pub alignment: NonZeroU32,
}

impl DataHalfs {
    pub fn size(&self) -> usize {
        let len = self.halfs.1.len();
        let alignment = self.alignment.get() as usize;
        let padding = if alignment > 2 {
            alignment - 2
        } else {
            0
        };
        let unit_size = padding + 2;
        len * unit_size
    }
}

#[derive(Clone, Debug)]
pub struct DataWords {
    pub words: (Option<Address>, Vec<u32>),
    pub alignment: NonZeroU32,
}

impl DataWords {
    pub fn size(&self) -> usize {
        let len = self.words.1.len();
        let alignment = self.alignment.get() as usize;
        let padding = if alignment > 4 {
            alignment - 4
        } else {
            0
        };
        let unit_size = padding + 4;
        len * unit_size
    }
}

#[derive(Clone, Debug)]
pub struct DataSpace {
    pub spaces: (Option<Address>, Vec<u8>),
}

impl DataSpace {
    pub fn size(&self) -> usize {
        self.spaces.1.len()
    }
}

#[derive(Clone, Debug)]
pub enum DataEntry {
    CString(DataCString),
    Bytes(DataBytes),
    Halfs(DataHalfs),
    Words(DataWords),
    Space(DataSpace),
}

impl DataEntry {
    pub fn size(&self) -> usize {
        match self {
            Self::CString(c) => c.size(),
            Self::Bytes(b) => b.size(),
            Self::Halfs(h) => h.size(),
            Self::Words(w) => w.size(),
            Self::Space(s) => s.size(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DataSegment {
    pub data_entries: Vec<DataEntry>,
    pub start_address: Option<Address>,
}

impl DataSegment {
    pub fn new() -> DataSegment {
        DataSegment {
            data_entries: Vec::new(),
            start_address: None,
        }
    }

    pub fn size(&self) -> usize {
        self.data_entries
            .iter()
            .fold(0, |acc, d| acc + d.size())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Parsed {
    pub text_segment: Vec<TextSegment>,
    pub ktext_segment: Vec<TextSegment>,
    pub data_segment: Vec<DataSegment>,
    pub kdata_segment: Vec<DataSegment>,
}

fn i_extract_imm<T>(imm: (Option<&str>, Result<T, ParseIntError>)) -> Option<T>
where
    T: std::ops::MulAssign + From<i8>,
{
    let mut imm_int: T = match imm.1 {
        Ok(i) => i as T,
        Err(_) => return None,
    };
    if let Some(sign) = imm.0 {
        if sign == "-" {
            imm_int *= T::from(-1i8);
        }
    };
    Some(imm_int)
}

fn parse_label(
    current_line: &str,
) -> Result<(String, Option<&str>), Err<(&str, nom::error::ErrorKind)>> {
    let (rest, l) = new_label(current_line)?;
    let l = String::from_iter(l.into_iter());
    let rest = rest.trim();
    if !rest.is_empty() && !entire_line_is_comment(rest) {
        Ok((l, Some(rest)))
    } else {
        Ok((l, None))
    }
}

fn check_directive<'a>(line: &'a str) -> Option<ParsedDirective<'a>> {
    if let Ok(align) = directive_align(line) {
        return Some(ParsedDirective::Align(Ok(align)));
    }
    if let Ok(ascii) = directive_ascii(line) {
        return Some(ParsedDirective::Ascii(Ok(ascii)));
    }
    if let Ok(asciiz) = directive_asciiz(line) {
        return Some(ParsedDirective::Asciiz(Ok(asciiz)));
    }
    if let Ok(byte) = directive_byte(line) {
        return Some(ParsedDirective::Byte(Ok(byte)));
    }
    if let Ok(data) = directive_data(line) {
        return Some(ParsedDirective::Data(Ok(data)));
    }
    if let Ok(half) = directive_half(line) {
        return Some(ParsedDirective::Half(Ok(half)));
    }
    if let Ok(kdata) = directive_kdata(line) {
        return Some(ParsedDirective::KData(Ok(kdata)));
    }
    if let Ok(ktext) = directive_ktext(line) {
        return Some(ParsedDirective::KText(Ok(ktext)));
    }
    if let Ok(space) = directive_space(line) {
        return Some(ParsedDirective::Space(Ok(space)));
    }
    if let Ok(text) = directive_text(line) {
        return Some(ParsedDirective::Text(Ok(text)));
    }
    if let Ok(word) = directive_word(line) {
        return Some(ParsedDirective::Word(Ok(word)));
    }
    return None;
}

fn parse_directive<'a>(
    mut current_line: &'a str,
    lines: &'a mut Lines,
) -> Option<ParsedDirective<'a>> {
    current_line = current_line.trim();
    if !(current_line.is_empty() || entire_line_is_comment(current_line)) {
        return check_directive(current_line);
    }
    for line in lines {
        let line = line.trim();
        if line.is_empty() || entire_line_is_comment(line) {
            continue;
        }
        return check_directive(line);
    }
    None
}

fn parse_text_segment(lines: &mut Lines, text_segment: &mut TextSegment) -> Option<String> {
    let mut current_labels: Option<Vec<String>> = None;
    for line in lines {
        let mut line = line.trim();
        if line.is_empty() || entire_line_is_comment(line) {
            continue;
        }
        if let Ok((l, rest)) = parse_label(line) {
            match current_labels {
                Some(ref mut v) => v.push(l),
                None => current_labels = Some(vec![l]),
            };
            if let Some(rest) = rest {
                line = rest;
            } else {
                continue;
            }
        }
        if let Ok((_, (inst, rd, rs, rt))) = r_arithmetic(line) {
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            text_segment.instructions.push((
                addr,
                RType::new(
                    RInst::from(inst),
                    Reg::from(rs),
                    Reg::from(rt),
                    Reg::from(rd),
                    0,
                )
                .into(),
            ));
            continue;
        }
        if let Ok((_, (inst, rd, rt, shamt))) = r_shift(line) {
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            if let Some(sign) = shamt.0 {
                if sign == "-" {
                    panic!("Cannot have negative shift amount: {}", line);
                }
            }
            let shamt_int = match shamt.1 {
                Ok(i) => i as u8,
                Err(p) => panic!("Unable to parse shift amount: {} because {}", line, p),
            };
            text_segment.instructions.push((
                addr,
                RType::new(
                    RInst::from(inst),
                    Reg::zero,
                    Reg::from(rt),
                    Reg::from(rd),
                    shamt_int,
                )
                .into(),
            ));
            continue;
        }
        if let Ok((_, (inst, rs))) = r_jump(line) {
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            text_segment.instructions.push((
                addr,
                RType::new(RInst::from(inst), Reg::from(rs), Reg::zero, Reg::zero, 0).into(),
            ));
            continue;
        }
        if let Ok((_, (inst, rt, rs, imm))) = i_arith(line) {
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            let imm_int = match i_extract_imm(imm) {
                Some(i) => i as u16,
                None => panic!("Unable to parse immediate: {}", line),
            };
            text_segment.instructions.push((
                addr,
                ITypeImm::new(IInst::from(inst), Reg::from(rs), Reg::from(rt), imm_int).into(),
            ));
            continue;
        }
        if let Ok((_, (inst, rt, rs, imm))) = i_branch_imm(line) {
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            let imm_int = match i_extract_imm(imm) {
                Some(i) => i as u16,
                None => panic!("Unable to parse immediate: {}", line),
            };
            text_segment.instructions.push((
                addr,
                ITypeImm::new(IInst::from(inst), Reg::from(rs), Reg::from(rt), imm_int).into(),
            ));
            continue;
        }
        if let Ok((_, (inst, rt, rs, label))) = i_branch_label(line) {
            let label = String::from_iter(label.into_iter());
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            text_segment.instructions.push((
                addr,
                ITypeLabel::new(
                    IInst::from(inst),
                    Reg::from(rs),
                    Reg::from(rt),
                    Address::from(label),
                )
                .into(),
            ));
            continue;
        }
        if let Ok((_, (inst, rt, imm, rs))) = i_mem_imm(line) {
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            let imm_int = match i_extract_imm(imm) {
                Some(i) => i as u16,
                None => panic!("Unable to parse immediate: {}", line),
            };
            text_segment.instructions.push((
                addr,
                ITypeImm::new(IInst::from(inst), Reg::from(rs), Reg::from(rt), imm_int).into(),
            ));
            continue;
        }
        if let Ok((_, (inst, rt, label, rs))) = i_mem_label(line) {
            let label = String::from_iter(label.into_iter());
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            text_segment.instructions.push((
                addr,
                ITypeLabel::new(
                    IInst::from(inst),
                    Reg::from(rs),
                    Reg::from(rt),
                    Address::from(label),
                )
                .into(),
            ));
            continue;
        }
        if let Ok((_, (inst, rt, imm))) = i_load_imm(line) {
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            let imm_int = match i_extract_imm(imm) {
                Some(i) => i as u16,
                None => panic!("Unable to parse immediate: {}", line),
            };
            text_segment.instructions.push((
                addr,
                ITypeImm::new(IInst::from(inst), Reg::zero, Reg::from(rt), imm_int).into(),
            ));
            continue;
        }
        if let Ok((_, (inst, rt, label))) = i_load_label(line) {
            let label = String::from_iter(label.into_iter());
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            text_segment.instructions.push((
                addr,
                ITypeLabel::new(
                    IInst::from(inst),
                    Reg::zero,
                    Reg::from(rt),
                    Address::from(label),
                )
                .into(),
            ));
            continue;
        }
        if let Ok((_, (inst, label))) = j_label(line) {
            let label = String::from_iter(label.into_iter());
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            text_segment.instructions.push((
                addr,
                JTypeLabel::new(JInst::from(inst), Address::from(label)).into(),
            ));
            continue;
        }
        if let Ok((_, inst)) = j_other(line) {
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            text_segment.instructions.push((
                addr,
                JTypeImm::new(JInst::from(inst), 0).into(),
            ));
            continue;
        }
        // it may be a new directive
        return Some(line.to_owned());
    }
    None
}

#[derive(Copy, Clone, Debug)]
enum Alignment {
    Defined(NonZeroU32),
    Automatic,
}

fn auto_align_data_segment(data_segment: &mut DataSegment, alignment: u32) {
    if let Some(start) = &data_segment.start_address {
        if let Some(numeric) = start.numeric {
            let r = numeric.get() % alignment;
            if r != 0 {
                let data_space = DataSpace {
                    spaces: (Some(start.clone()), vec![0u8; r as usize]),
                };
                data_segment.data_entries.push(DataEntry::Space(data_space));
            }
        } else {
            std::unreachable!("Data segment start address was not a numeric");
        }
    }
}

fn parse_data_segment(lines: &mut Lines, data_segment: &mut DataSegment) -> Option<String> {
    let mut current_labels: Option<Vec<String>> = None;
    let mut current_alignment = Alignment::Automatic;
    for line in lines {
        let mut line = line.trim();
        if line.is_empty() || entire_line_is_comment(line) {
            continue;
        }
        if let Ok((l, rest)) = parse_label(line) {
            match current_labels {
                Some(ref mut v) => v.push(l),
                None => current_labels = Some(vec![l]),
            };
            if let Some(rest) = rest {
                line = rest;
            } else {
                continue;
            }
        }
        if let Ok((_, imm)) = directive_align(line) {
            let imm_int = match i_extract_imm(imm) {
                Some(i) => i,
                None => panic!("Unable to parse immediate for align directive: {}", line),
            };
            if imm_int < 0 {
                panic!("Cannot have negative aligment: {}", line);
            }
            if imm_int > 31 {
                panic!("Cannot have alignment >= 2^32: 2^{}", imm_int);
            }
            current_alignment = Alignment::Defined(unsafe {
                NonZeroU32::new_unchecked(1u32 << imm_int)
            });
            if data_segment.data_entries.len() == 0 {
                auto_align_data_segment(data_segment, 1u32 << imm_int);
            }
            continue;
        }
        // if the first directive after .data is not .align
        // then we ensure that we are aligned on an 8 byte boundary
        // Otherwise .half, .word, ..., could be misaligned
        if data_segment.data_entries.len() == 0 {
            auto_align_data_segment(data_segment, 8);
        }
        // Everything below relies on this
        let align = current_alignment;
        if let Alignment::Defined(a) = current_alignment {
            if a.get() != 1 {
                // 2^0 applies until next .data
                current_alignment = Alignment::Automatic;
            }
        };
        if let Ok((_, s)) = directive_ascii(line) {
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            let align = match align {
                Alignment::Defined(a) => a,
                Alignment::Automatic => unsafe { NonZeroU32::new_unchecked(1) },
            };
            let cstring = DataCString {
                chars: (addr, s.as_bytes().to_vec()), // Rust strings aren't null terminated,
                alignment: align,
                null_terminated: false,
            };
            data_segment.data_entries.push(DataEntry::CString(cstring));
            continue;
        }
        if let Ok((_, s)) = directive_asciiz(line) {
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            let align = match align {
                Alignment::Defined(a) => a,
                Alignment::Automatic => unsafe { NonZeroU32::new_unchecked(1) },
            };
            let mut cstring = DataCString {
                chars: (addr, s.as_bytes().to_vec()), // Rust strings aren't null terminated,
                alignment: align,
                null_terminated: true,
            };
            cstring.chars.1.push(0);
            data_segment.data_entries.push(DataEntry::CString(cstring));
            continue;
        }
        if let Ok((_, bytes)) = directive_byte(line) {
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            let align = match align {
                Alignment::Defined(a) => a,
                Alignment::Automatic => unsafe { NonZeroU32::new_unchecked(1) },
            };
            let mut byte_vec = Vec::new();
            for entry in bytes {
                let imm = match i_extract_imm(entry) {
                    Some(b) => b as u8,
                    None => panic!("Syntax error in byte directive: {}", line),
                };
                byte_vec.push(imm);
            }
            let data_bytes = DataBytes {
                bytes: (addr, byte_vec),
                alignment: align,
            };
            data_segment.data_entries.push(DataEntry::Bytes(data_bytes));
            continue;
        }
        if let Ok((_, halfs)) = directive_half(line) {
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            let align = match align {
                Alignment::Defined(a) => a,
                Alignment::Automatic => unsafe { NonZeroU32::new_unchecked(2) },
            };
            let mut half_vec = Vec::new();
            for entry in halfs {
                let imm = match i_extract_imm(entry) {
                    Some(b) => b as u16,
                    None => panic!("Syntax error in half directive: {}", line),
                };
                half_vec.push(imm);
            }
            let data_halfs = DataHalfs {
                halfs: (addr, half_vec),
                alignment: align,
            };
            data_segment.data_entries.push(DataEntry::Halfs(data_halfs));
            continue;
        }
        if let Ok((_, words)) = directive_word(line) {
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            let align = match align {
                Alignment::Defined(a) => a,
                Alignment::Automatic => unsafe { NonZeroU32::new_unchecked(4) },
            };
            let mut word_vec = Vec::new();
            for entry in words {
                let imm = match i_extract_imm(entry) {
                    Some(b) => b as u32,
                    None => panic!("Syntax error in word directive: {}", line),
                };
                word_vec.push(imm);
            }
            let data_words = DataWords {
                words: (addr, word_vec),
                alignment: align,
            };
            data_segment.data_entries.push(DataEntry::Words(data_words));
            continue;
        }
        if let Ok((_, imm)) = directive_space(line) {
            let addr = current_labels.map_or_else(|| None, |v| Some(Address::from(v.as_slice())));
            current_labels = None;
            let imm = match i_extract_imm(imm) {
                Some(i) => i as u32,
                None => panic!("Expected amount of space after space directive: {}", line),
            };
            let data_space = DataSpace {
                spaces: (addr, vec![0u8; imm as usize]),
            };
            data_segment.data_entries.push(DataEntry::Space(data_space));
            continue;
        }
        // it may be a new directive
        return Some(line.to_owned());
    }
    None
}

pub fn parse(program: &str) -> Parsed {
    let mut parsed = Parsed::default();
    let mut lines = program.lines();

    let mut line: String;
    match lines.next() {
        Some(l) => line = l.to_string(),
        None => return parsed,
    }

    loop {
        match parse_directive(&line, &mut lines) {
            Some(ParsedDirective::Text(Ok((_, imm)))) => {
                let mut text_segment = TextSegment::new();

                if let Some(imm) = imm {
                    match i_extract_imm(imm) {
                        Some(i) => {
                            if i % 4 != 0 {
                                panic!("Text segment must be word-aligned");
                            }
                            text_segment.start_address =
                                Some(Address::new(NonZeroU32::new(i as u32), None))
                        }
                        None => (),
                    }
                }

                let more_lines = parse_text_segment(&mut lines, &mut text_segment);

                if text_segment.instructions.len() > 0 {
                    parsed.text_segment.push(text_segment);
                }

                match more_lines {
                    Some(l) => line = l,
                    None => break,
                }
            }
            Some(ParsedDirective::KText(Ok((_, imm)))) => {
                let mut text_segment = TextSegment::new();

                if let Some(imm) = imm {
                    match i_extract_imm(imm) {
                        Some(i) => {
                            if i % 4 != 0 {
                                panic!("Text segment must be word-aligned");
                            }
                            text_segment.start_address =
                                Some(Address::new(NonZeroU32::new(i as u32), None))
                        }
                        None => (),
                    }
                }

                let more_lines = parse_text_segment(&mut lines, &mut text_segment);

                if text_segment.instructions.len() > 0 {
                    parsed.ktext_segment.push(text_segment);
                }

                match more_lines {
                    Some(l) => line = l,
                    None => break,
                }
            }
            Some(ParsedDirective::Data(Ok((_, imm)))) => {
                let mut data_segment = DataSegment::new();

                if let Some(imm) = imm {
                    match i_extract_imm(imm) {
                        Some(i) => {
                            data_segment.start_address =
                                Some(Address::new(NonZeroU32::new(i as u32), None))
                        }
                        None => (),
                    }
                }

                let more_lines = parse_data_segment(&mut lines, &mut data_segment);

                if data_segment.data_entries.len() > 0 {
                    parsed.data_segment.push(data_segment);
                }

                match more_lines {
                    Some(l) => line = l,
                    None => break,
                }
            }
            Some(ParsedDirective::KData(Ok((_, imm)))) => {
                let mut data_segment = DataSegment::new();

                if let Some(imm) = imm {
                    match i_extract_imm(imm) {
                        Some(i) => {
                            data_segment.start_address =
                                Some(Address::new(NonZeroU32::new(i as u32), None))
                        }
                        None => (),
                    }
                }

                let more_lines = parse_data_segment(&mut lines, &mut data_segment);

                if data_segment.data_entries.len() > 0 {
                    parsed.kdata_segment.push(data_segment);
                }

                match more_lines {
                    Some(l) => line = l,
                    None => break,
                }
            }
            Some(_) => panic!("Unexpected Directive: {}", line),
            None => break,
        }
    }
    parsed
}
