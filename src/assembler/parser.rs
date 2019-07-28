use nom::{IResult,
          branch::{alt},
          bytes::complete::{tag},
          character::complete::{digit1,
                                hex_digit1,
                                not_line_ending,
                                alphanumeric1},
          combinator::{opt, map},
          sequence::{pair,
                     preceded,
                     terminated},
};

use std::vec::Vec;
use std::str::*;
use std::num::ParseIntError;

#[derive(Clone, Debug)]
pub struct Parsed {
    pub data_segment:  Option<Segment>,
    pub text_segment:  Option<Segment>,
    pub kdata_segment: Option<Segment>,
    pub ktext_segment: Option<Segment>,
}

impl Parsed {
    pub fn new(data_segment:  Option<Segment>,
               text_segment:  Option<Segment>,
               kdata_segment: Option<Segment>,
               ktext_segment: Option<Segment>) -> Parsed {
        Parsed {data_segment, text_segment, kdata_segment, ktext_segment}
    }

    pub fn default() -> Parsed {
        Parsed {
            data_segment:  None,
            text_segment:  None,
            kdata_segment: None,
            ktext_segment: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Segment {
    start: u32,
    entries: Vec<SegmentEntry>,
}

impl Segment {
    pub fn new(start: u32, entries: Vec<SegmentEntry>) -> Segment {
        Segment {start, entries}
    }

    pub fn add_entry(&mut self, entry: SegmentEntry) {
        self.entries.push(entry);
    }

    pub fn size(&self) -> u32 {
        self.entries.iter().fold(0, |acc, s| acc + s.size())
    }
}

#[derive(Clone, Debug)]
pub struct SegmentEntry {
    offset: u32,
    label: Option<String>,
    alignment: Alignment,
    data: Vec<u32>,
}

impl SegmentEntry {
    pub fn new(offset: u32,
               label: Option<String>,
               alignment: Alignment,
               data: Vec<u32>) -> SegmentEntry {
        SegmentEntry {offset, label, alignment, data}
    }

    pub fn size(&self) -> u32 {
        (self.data.len() * usize::from(self.alignment)) as u32
    }

    pub fn add_data(&mut self, data: u32) {
        self.data.push(data)
    }

    pub fn get_element_offset(&self, element: u32) -> u32 {
        element * u32::from(self.alignment)
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

fn sign(input: &str) -> IResult<&str, &str> {
    alt((tag("+"), tag("-")))(input)
}

macro_rules! gen_nom_ints_dec {
    ($name: ident, $type: ty) => {
        fn $name(input: &str) -> IResult<&str, (Option<&str>, Result<$type, ParseIntError>)> {
            pair(opt(sign), map(digit1, |s: &str| FromStr::from_str(s)))(input)
        }
    };
}

macro_rules! gen_nom_ints_hex {
    ($name: ident, $type: ty) => {
        fn $name(input: &str) -> IResult<&str, (Option<&str>, Result<$type, ParseIntError>)> {
            pair(opt(sign), preceded(tag("0x"), map(hex_digit1, |s: &str| <$type>::from_str_radix(s, 16))))(input)
        }
    };
}

gen_nom_ints_dec!(parse_dec_int8, i8);
gen_nom_ints_dec!(parse_dec_int16, i16);
gen_nom_ints_dec!(parse_dec_int32, i32);
gen_nom_ints_dec!(parse_dec_int64, i64);
gen_nom_ints_dec!(parse_dec_int128, i128);
gen_nom_ints_hex!(parse_hex_int8, i8);
gen_nom_ints_hex!(parse_hex_int16, i16);
gen_nom_ints_hex!(parse_hex_int32, i32);
gen_nom_ints_hex!(parse_hex_int64, i64);
gen_nom_ints_hex!(parse_hex_int128, i128);

fn v_reg_name(input: &str) -> IResult<&str, &str> {
    preceded(tag("v"), alt((tag("0"), tag("1"))))(input)
}

fn v_reg_num(input: &str) -> IResult<&str, &str> {
    alt((tag("2"), tag("3")))(input)
}

fn a_reg_name(input: &str) -> IResult<&str, &str> {
    preceded(tag("a"), alt((tag("0"),
                            tag("1"),
                            tag("2"),
                            tag("3"))
                           )
             )(input)
}

fn a_reg_num(input: &str) -> IResult<&str, &str> {
    alt((tag("4"),
         tag("5"),
         tag("6"),
         tag("7"))
        )(input)
}

fn t_reg_name(input: &str) -> IResult<&str, &str> {
    preceded(tag("t"), alt((tag("0"),
                            tag("1"),
                            tag("2"),
                            tag("3"),
                            tag("4"),
                            tag("5"),
                            tag("6"),
                            tag("7"),
                            tag("8"),
                            tag("9"))
                           )
             )(input)
}

fn t_reg_num(input: &str) -> IResult<&str, &str> {
    alt((tag("8"),
         tag("9"),
         tag("10"),
         tag("11"),
         tag("12"),
         tag("13"),
         tag("14"),
         tag("15"),
         tag("24"),
         tag("25"))
        )(input)
}

fn s_reg_name(input: &str) -> IResult<&str, &str> {
    preceded(tag("s"), alt((tag("0"),
                            tag("1"),
                            tag("2"),
                            tag("3"),
                            tag("4"),
                            tag("5"),
                            tag("6"),
                            tag("7"))
                           )
             )(input)
}

fn s_reg_num(input: &str) -> IResult<&str, &str> {
    alt((tag("16"),
         tag("17"),
         tag("18"),
         tag("19"),
         tag("20"),
         tag("21"),
         tag("22"),
         tag("23"))
        )(input)
}

fn register_named(input: &str) -> IResult<&str, &str> {
    preceded(tag("$"), alt((tag("zero"),
                            tag("at"),
                            tag("sp"),
                            tag("fp"),
                            tag("ra"),
                            v_reg_name,
                            a_reg_name,
                            t_reg_name,
                            s_reg_name))
             )(input)
}

fn register_numbered(input: &str) -> IResult<&str, &str> {
    preceded(tag("$"), alt((tag("0"),
                            tag("1"),
                            tag("29"),
                            tag("30"),
                            tag("31"),
                            v_reg_num,
                            a_reg_num,
                            t_reg_num,
                            s_reg_num))
             )(input)
}

fn single_line_comment(input: &str) -> IResult<&str, &str> {
    preceded(tag("#"), not_line_ending)(input)
}

fn label(input: &str) -> IResult<&str, &str> {
    terminated(alphanumeric1, tag(":"))(input)
}

fn directive(input: &str) -> IResult<&str, &str> {
    preceded(tag("."), alphanumeric1)(input)
}

fn r_mnemonic(input: &str) -> IResult<&str, &str> {
    alt((tag("add"),
         tag("addu"),
         tag("and"),
         tag("jr"),
         tag("nor"),
         tag("or"),
         tag("slt"),
         tag("sltu"),
         tag("sll"),
         tag("srl"),
         tag("sub"),
         tag("subu"),
         tag("div"),
         tag("divu")
         ))(input)
}

fn i_mnemonic(input: &str) -> IResult<&str, &str> {
    alt((tag("addi"),
         tag("addiu"),
         tag("andi"),
         tag("beq"),
         tag("bne"),
         tag("lbu"),
         tag("lhu"),
         tag("ll"),
         tag("li"),
         tag("la"),
         tag("lui"),
         tag("lw"),
         tag("ori"),
         tag("slti"),
         tag("sltiu"),
         tag("sb"),
         tag("sc"),
         tag("sh"),
         tag("sw"))
        )(input)
}

fn j_mnemonic(input: &str) -> IResult<&str, &str> {
    alt((tag("j"), tag("jal")))(input)
}

pub fn parse(program: &str) -> Parsed {
    let mut parsed = Parsed::default();
    parsed
}
