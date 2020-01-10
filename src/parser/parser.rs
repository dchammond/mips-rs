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
}

#[derive(Clone, Debug)]
pub struct KTextSegment {
    pub instructions: Vec<(Option<Address>, Inst)>,
    pub start_address: Option<Address>,
}

impl From<TextSegment> for KTextSegment {
    fn from(t: TextSegment) -> Self {
        KTextSegment {
            instructions: t.instructions,
            start_address: t.start_address,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct DataAlignment {
    pub alignment: u32,
}

#[derive(Clone, Debug)]
pub struct DataCString {
    pub chars: (Option<Vec<Address>>, Vec<u8>),
    pub null_terminated: bool,
}

#[derive(Clone, Debug)]
pub struct DataBytes {
    pub bytes: (Option<Vec<Address>>, Vec<u8>),
}

#[derive(Clone, Debug)]
pub struct DataHalfs {
    pub halfs: (Option<Vec<Address>>, Vec<u16>),
}

#[derive(Clone, Debug)]
pub struct DataWords {
    pub words: (Option<Vec<Address>>, Vec<u32>),
}

#[derive(Clone, Debug)]
pub struct DataSpace {
    pub spaces: (Option<Vec<Address>>, Vec<u8>),
}

#[derive(Clone, Debug)]
pub enum DataEntry {
    Alignment(DataAlignment),
    CString(DataCString),
    Bytes(DataBytes),
    Halfs(DataHalfs),
    Words(DataWords),
    Space(DataSpace),
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
}

#[derive(Clone, Debug)]
pub struct KDataSegment {
    pub data_entries: Vec<DataEntry>,
    pub start_address: Option<Address>,
}

impl From<DataSegment> for KDataSegment {
    fn from(d: DataSegment) -> Self {
        KDataSegment {
            data_entries: d.data_entries,
            start_address: d.start_address,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Parsed {
    pub text_segment: Vec<TextSegment>,
    pub ktext_segment: Vec<KTextSegment>,
    pub data_segment: Vec<DataSegment>,
    pub kdata_segment: Vec<KDataSegment>,
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

fn parse_directive<'a>(current_line: &'a str, lines: &'a mut Lines) -> Option<ParsedDirective<'a>> {
    let mut first = Some(current_line);
    for line in lines {
        let line = match first {
            Some(f) => {
                first = None;
                f.trim()
            }
            None => line.trim(),
        };
        if line.is_empty() || entire_line_is_comment(line) {
            continue;
        }
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
                JType::new(JInst::from(inst), Address::from(label)).into(),
            ));
            continue;
        }
        // it may be a new directive
        return Some(line.to_owned());
    }
    None
}

fn parse_data_segment(lines: &mut Lines, data_segment: &mut DataSegment) -> Option<String> {
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
        if let Ok((_, imm)) = directive_align(line) {
            let imm_int = match i_extract_imm(imm) {
                Some(i) => i,
                None => panic!("Unable to parse immediate for align directive: {}", line),
            };
            if imm_int < 0 {
                panic!("Cannot have negative aligment: {}", line);
            }
            let imm_int = imm_int as u32;
            if imm_int & (imm_int - 1) != 0 {
                panic!("Alignment must be power of 2: {}", line);
            }
            let alignment = DataAlignment { alignment: imm_int };
            data_segment
                .data_entries
                .push(DataEntry::Alignment(alignment));
            continue;
        }
        if let Ok((_, s)) = directive_ascii(line) {
            let addr = current_labels.map_or_else(
                || None,
                |v| Some(v.into_iter().map(|s| Address::from(s)).collect()),
            );
            current_labels = None;
            let cstring = DataCString {
                chars: (addr, s.as_bytes().to_vec()), // Rust strings aren't null terminated
                null_terminated: false,
            };
            data_segment.data_entries.push(DataEntry::CString(cstring));
            continue;
        }
        if let Ok((_, s)) = directive_asciiz(line) {
            let addr = current_labels.map_or_else(
                || None,
                |v| Some(v.into_iter().map(|s| Address::from(s)).collect()),
            );
            current_labels = None;
            let mut cstring = DataCString {
                chars: (addr, s.as_bytes().to_vec()), // Rust strings aren't null terminated
                null_terminated: true,
            };
            cstring.chars.1.push(0);
            data_segment.data_entries.push(DataEntry::CString(cstring));
            continue;
        }
        if let Ok((_, bytes)) = directive_byte(line) {
            let addr = current_labels.map_or_else(
                || None,
                |v| Some(v.into_iter().map(|s| Address::from(s)).collect()),
            );
            current_labels = None;
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
            };
            data_segment.data_entries.push(DataEntry::Bytes(data_bytes));
            continue;
        }
        if let Ok((_, halfs)) = directive_half(line) {
            let addr = current_labels.map_or_else(
                || None,
                |v| Some(v.into_iter().map(|s| Address::from(s)).collect()),
            );
            current_labels = None;
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
            };
            data_segment.data_entries.push(DataEntry::Halfs(data_halfs));
            continue;
        }
        if let Ok((_, words)) = directive_word(line) {
            let addr = current_labels.map_or_else(
                || None,
                |v| Some(v.into_iter().map(|s| Address::from(s)).collect()),
            );
            current_labels = None;
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
            };
            data_segment.data_entries.push(DataEntry::Words(data_words));
            continue;
        }
        if let Ok((_, imm)) = directive_space(line) {
            let addr = current_labels.map_or_else(
                || None,
                |v| Some(v.into_iter().map(|s| Address::from(s)).collect()),
            );
            current_labels = None;
            let imm = match i_extract_imm(imm) {
                Some(i) => i as u32,
                None => panic!("Expected amount of space after space directive: {}", line),
            };
            let data_space = DataSpace {
                spaces: (addr, vec![0; imm as usize]),
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
                            text_segment.start_address =
                                Some(Address::new(NonZeroU32::new(i as u32), None))
                        }
                        None => (),
                    }
                }

                let more_lines = parse_text_segment(&mut lines, &mut text_segment);

                if text_segment.instructions.len() > 0 {
                    parsed.ktext_segment.push(KTextSegment::from(text_segment));
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
                    parsed.kdata_segment.push(KDataSegment::from(data_segment));
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
