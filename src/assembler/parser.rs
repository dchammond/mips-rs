use nom::*;

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

named!(sign<&str, &str>,
    alt!(tag_s!("+") | tag_s!("-"))
);

macro_rules! gen_nom_ints_dec {
    ($name: ident, $type: ty) => {
        named!($name<&str, (Option<&str>, Result<$type, ParseIntError>)>,
            pair!(opt!(sign), map!(digit, FromStr::from_str))
        );
    };
}

macro_rules! gen_nom_ints_hex {
    ($name: ident, $type: ty) => {
        named!($name<&str, (Option<&str>, Result<$type, ParseIntError>)>,
            tuple!(opt!(sign), preceded!(tag_s!("0x"), map!(digit, |s| <$type>::from_str_radix(s, 16))))
        );
    };
}

named!(v_reg_name<&str, &str>,
    preceded!(tag_s!("v"), alt!(tag_s!("0") |
                                tag_s!("1"))
    )
);

named!(v_reg_num<&str, &str>,
    alt!(tag_s!("2") | tag_s!("3"))
);

named!(a_reg_name<&str, &str>,
    preceded!(tag_s!("a"), alt!(tag_s!("0") |
                                tag_s!("1") |
                                tag_s!("2") |
                                tag_s!("3"))
    )
);

named!(a_reg_num<&str, &str>,
    alt!(tag_s!("4") | tag_s!("5") | tag_s!("6") | tag_s!("7"))
);

named!(t_reg_name<&str, &str>,
    preceded!(tag_s!("t"), alt!(tag_s!("0") |
                                tag_s!("1") |
                                tag_s!("2") |
                                tag_s!("3") |
                                tag_s!("4") |
                                tag_s!("5") |
                                tag_s!("6") |
                                tag_s!("7") |
                                tag_s!("8") |
                                tag_s!("9"))
    )
);

named!(t_reg_num<&str, &str>,
    alt!(tag_s!("8")  |
         tag_s!("9")  |
         tag_s!("10") |
         tag_s!("11") |
         tag_s!("12") |
         tag_s!("13") |
         tag_s!("14") |
         tag_s!("15") |
         tag_s!("24") |
         tag_s!("25"))
);

named!(s_reg_name<&str, &str>,
    preceded!(tag_s!("s"), alt!(tag_s!("0") |
                                tag_s!("1") |
                                tag_s!("2") |
                                tag_s!("3") |
                                tag_s!("4") |
                                tag_s!("5") |
                                tag_s!("6") |
                                tag_s!("7"))
    )
);

named!(s_reg_num<&str, &str>,
    alt!(tag_s!("16") |
         tag_s!("17") |
         tag_s!("18") |
         tag_s!("19") |
         tag_s!("20") |
         tag_s!("21") |
         tag_s!("22") |
         tag_s!("23"))
);

named!(register_named<&str, &str>,
    preceded!(tag_s!("$"), alt!(tag_s!("zero") |
                                tag_s!("at")   |
                                tag_s!("sp")   |
                                tag_s!("fp")   |
                                tag_s!("ra")   |
                                v_reg_name     |
                                a_reg_name     |
                                t_reg_name     |
                                s_reg_name)
    )
);

named!(register_numbered<&str, &str>,
    preceded!(tag_s!("$"), alt!(tag_s!("0") |
                                tag_s!("1")  |
                                tag_s!("29") |
                                tag_s!("30") |
                                tag_s!("31") |
                                v_reg_num    |
                                a_reg_num    |
                                t_reg_num    |
                                s_reg_num)
    )
);

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

pub fn parse(program: &str) -> Parsed {
    let mut parsed = Parsed::default();
    parsed
}
