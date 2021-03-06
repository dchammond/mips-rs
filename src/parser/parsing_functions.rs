#![allow(dead_code)]

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, hex_digit1, not_line_ending, space0, space1},
    combinator::{map, opt, recognize},
    multi::{many1, separated_nonempty_list},
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

use std::num::ParseIntError;
use std::str::FromStr;

pub fn sign(input: &str) -> IResult<&str, &str> {
    alt((tag("+"), tag("-")))(input)
}

pub fn identifier(input: &str) -> IResult<&str, Vec<&str>> {
    many1(alt((alphanumeric1, tag("_"))))(input)
}

macro_rules! gen_nom_ints_dec {
    ($name: ident, $type: ty) => {
        pub fn $name(input: &str) -> IResult<&str, (Option<&str>, Result<$type, ParseIntError>)> {
            pair(opt(sign), map(digit1, |s: &str| FromStr::from_str(s)))(input)
        }
    };
}

macro_rules! gen_nom_ints_hex {
    ($name: ident, $type: ty) => {
        pub fn $name(input: &str) -> IResult<&str, (Option<&str>, Result<$type, ParseIntError>)> {
            pair(
                opt(sign),
                preceded(
                    tag("0x"),
                    map(hex_digit1, |s: &str| <$type>::from_str_radix(s, 16)),
                ),
            )(input)
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

pub fn v_reg_name(input: &str) -> IResult<&str, &str> {
    preceded(tag("v"), alt((tag("0"), tag("1"))))(input)
}

pub fn v_reg_num(input: &str) -> IResult<&str, &str> {
    alt((tag("2"), tag("3")))(input)
}

pub fn a_reg_name(input: &str) -> IResult<&str, &str> {
    preceded(tag("a"), alt((tag("0"), tag("1"), tag("2"), tag("3"))))(input)
}

pub fn a_reg_num(input: &str) -> IResult<&str, &str> {
    alt((tag("4"), tag("5"), tag("6"), tag("7")))(input)
}

pub fn t_reg_name(input: &str) -> IResult<&str, &str> {
    preceded(
        tag("t"),
        alt((
            tag("0"),
            tag("1"),
            tag("2"),
            tag("3"),
            tag("4"),
            tag("5"),
            tag("6"),
            tag("7"),
            tag("8"),
            tag("9"),
        )),
    )(input)
}

pub fn t_reg_num(input: &str) -> IResult<&str, &str> {
    alt((
        tag("8"),
        tag("9"),
        tag("10"),
        tag("11"),
        tag("12"),
        tag("13"),
        tag("14"),
        tag("15"),
        tag("24"),
        tag("25"),
    ))(input)
}

pub fn s_reg_name(input: &str) -> IResult<&str, &str> {
    preceded(
        tag("s"),
        alt((
            tag("0"),
            tag("1"),
            tag("2"),
            tag("3"),
            tag("4"),
            tag("5"),
            tag("6"),
            tag("7"),
        )),
    )(input)
}

pub fn s_reg_num(input: &str) -> IResult<&str, &str> {
    alt((
        tag("16"),
        tag("17"),
        tag("18"),
        tag("19"),
        tag("20"),
        tag("21"),
        tag("22"),
        tag("23"),
    ))(input)
}

pub fn register_named(input: &str) -> IResult<&str, &str> {
    recognize(preceded(
        tag("$"),
        alt((
            tag("zero"),
            tag("at"),
            tag("sp"),
            tag("fp"),
            tag("ra"),
            v_reg_name,
            a_reg_name,
            t_reg_name,
            s_reg_name,
        )),
    ))(input)
}

pub fn register_numbered(input: &str) -> IResult<&str, &str> {
    recognize(preceded(
        tag("$"),
        alt((
            tag("0"),
            tag("1"),
            tag("29"),
            tag("30"),
            tag("31"),
            v_reg_num,
            a_reg_num,
            t_reg_num,
            s_reg_num,
        )),
    ))(input)
}

pub fn register(input: &str) -> IResult<&str, &str> {
    alt((register_named, register_numbered))(input)
}

pub fn single_line_comment(input: &str) -> IResult<&str, &str> {
    preceded(tag("#"), not_line_ending)(input)
}

pub fn entire_line_is_comment(input: &str) -> bool {
    match input.get(0..1) {
        Some(c) => c == "#",
        None => false,
    }
}

pub fn new_label(input: &str) -> IResult<&str, Vec<&str>> {
    terminated(identifier, tag(":"))(input)
}

pub fn directive(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(tag("."), identifier)(input)
}

pub fn comma_space(input: &str) -> IResult<&str, &str> {
    preceded(tag(","), space0)(input)
}

pub fn r_arithmetic_mnemonic(input: &str) -> IResult<&str, &str> {
    alt((
        tag("add"),
        tag("addu"),
        tag("and"),
        tag("nor"),
        tag("or"),
        tag("slt"),
        tag("sltu"),
        tag("sub"),
        tag("subu"),
        tag("div"),
        tag("divu"),
    ))(input)
}

pub fn r_shift_mnemonic(input: &str) -> IResult<&str, &str> {
    alt((tag("sll"), tag("srl")))(input)
}

pub fn r_jump_mnemonic(input: &str) -> IResult<&str, &str> {
    tag("jr")(input)
}

pub fn i_arith_mnemonic(input: &str) -> IResult<&str, &str> {
    alt((
        tag("addi"),
        tag("addiu"),
        tag("andi"),
        tag("ori"),
        tag("slti"),
        tag("sltiu"),
    ))(input)
}

pub fn i_branch_mnemonic(input: &str) -> IResult<&str, &str> {
    alt((tag("beq"), tag("bne")))(input)
}

pub fn i_mem_mnemonic(input: &str) -> IResult<&str, &str> {
    alt((
        tag("lbu"),
        tag("lhu"),
        tag("ll"),
        tag("lw"),
        tag("sb"),
        tag("sc"),
        tag("sh"),
        tag("sw"),
    ))(input)
}

pub fn i_load_mnemonic(input: &str) -> IResult<&str, &str> {
    alt((tag("li"), tag("lui"), tag("la")))(input)
}

pub fn j_mnemonic(input: &str) -> IResult<&str, &str> {
    alt((tag("j"), tag("jal")))(input)
}

pub fn r_arithmetic(input: &str) -> IResult<&str, (&str, &str, &str, &str)> {
    tuple((
        terminated(r_arithmetic_mnemonic, space1),
        terminated(register, comma_space),
        terminated(register, comma_space),
        register,
    ))(input)
}

pub fn r_shift(
    input: &str,
) -> IResult<&str, (&str, &str, &str, (Option<&str>, Result<i8, ParseIntError>))> {
    tuple((
        terminated(r_shift_mnemonic, space1),
        terminated(register, comma_space),
        terminated(register, comma_space),
        alt((parse_hex_int8, parse_dec_int8)),
    ))(input)
}

pub fn r_jump(input: &str) -> IResult<&str, (&str, &str)> {
    pair(terminated(r_jump_mnemonic, space1), register)(input)
}

pub fn i_arith(
    input: &str,
) -> IResult<&str, (&str, &str, &str, (Option<&str>, Result<i64, ParseIntError>))> {
    tuple((
        terminated(i_arith_mnemonic, space1),
        terminated(register, comma_space),
        terminated(register, comma_space),
        alt((parse_hex_int64, parse_dec_int64)),
    ))(input)
}

pub fn i_branch_imm(
    input: &str,
) -> IResult<&str, (&str, &str, &str, (Option<&str>, Result<i64, ParseIntError>))> {
    tuple((
        terminated(i_branch_mnemonic, space1),
        terminated(register, comma_space),
        terminated(register, comma_space),
        alt((parse_hex_int64, parse_dec_int64)),
    ))(input)
}

pub fn i_branch_label(input: &str) -> IResult<&str, (&str, &str, &str, Vec<&str>)> {
    tuple((
        terminated(i_branch_mnemonic, space1),
        terminated(register, comma_space),
        terminated(register, comma_space),
        identifier,
    ))(input)
}

pub fn i_mem_imm(
    input: &str,
) -> IResult<&str, (&str, &str, (Option<&str>, Result<i64, ParseIntError>), &str)> {
    tuple((
        terminated(i_mem_mnemonic, space1),
        terminated(register, comma_space),
        terminated(alt((parse_hex_int64, parse_dec_int64)), tag("(")),
        terminated(register, tag(")")),
    ))(input)
}

pub fn i_mem_label(input: &str) -> IResult<&str, (&str, &str, Vec<&str>, &str)> {
    tuple((
        terminated(i_mem_mnemonic, space1),
        terminated(register, comma_space),
        terminated(identifier, tag("(")),
        terminated(register, tag(")")),
    ))(input)
}

pub fn i_load_imm(
    input: &str,
) -> IResult<&str, (&str, &str, (Option<&str>, Result<i64, ParseIntError>))> {
    tuple((
        terminated(i_load_mnemonic, space1),
        terminated(register, comma_space),
        alt((parse_hex_int64, parse_dec_int64)),
    ))(input)
}

pub fn i_load_label(input: &str) -> IResult<&str, (&str, &str, Vec<&str>)> {
    tuple((
        terminated(i_load_mnemonic, space1),
        terminated(register, comma_space),
        identifier,
    ))(input)
}

pub fn j_label(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    pair(terminated(j_mnemonic, space1), identifier)(input)
}

pub type DirectiveAlignResult<'a> = IResult<&'a str, (Option<&'a str>, Result<i64, ParseIntError>)>;

pub fn directive_align<'a>(input: &'a str) -> DirectiveAlignResult {
    preceded(
        tag("."),
        preceded(
            tag("align"),
            preceded(space1, alt((parse_hex_int64, parse_dec_int64))),
        ),
    )(input)
}

pub type DirectiveDataResult<'a> =
    IResult<&'a str, Option<(Option<&'a str>, Result<i64, ParseIntError>)>>;

pub fn directive_data<'a>(input: &'a str) -> DirectiveDataResult {
    preceded(
        tag("."),
        preceded(
            tag("data"),
            opt(preceded(space1, alt((parse_hex_int64, parse_dec_int64)))),
        ),
    )(input)
}

pub type DirectiveAsciiResult<'a> = IResult<&'a str, &'a str>;

pub fn directive_ascii<'a>(input: &'a str) -> DirectiveAsciiResult {
    preceded(
        tag("."),
        preceded(tag("ascii"), preceded(space1, not_line_ending)),
    )(input)
}

pub type DirectiveAsciizResult<'a> = IResult<&'a str, &'a str>;

pub fn directive_asciiz<'a>(input: &'a str) -> DirectiveAsciizResult {
    preceded(
        tag("."),
        preceded(tag("asciiz"), preceded(space1, not_line_ending)),
    )(input)
}

pub type DirectiveByteResult<'a> =
    IResult<&'a str, Vec<(Option<&'a str>, Result<i16, ParseIntError>)>>;

pub fn directive_byte<'a>(input: &'a str) -> DirectiveByteResult {
    preceded(
        tag("."),
        preceded(
            tag("byte"),
            preceded(
                space1,
                separated_nonempty_list(
                    alt((pair(tag(","), space0), pair(tag(""), space1))), // alt needs all options to return the same type
                    alt((parse_hex_int16, parse_dec_int16)),
                ),
            ),
        ),
    )(input)
}

pub type DirectiveHalfResult<'a> =
    IResult<&'a str, Vec<(Option<&'a str>, Result<i32, ParseIntError>)>>;

pub fn directive_half<'a>(input: &'a str) -> DirectiveHalfResult {
    preceded(
        tag("."),
        preceded(
            tag("half"),
            preceded(
                space1,
                separated_nonempty_list(
                    alt((pair(tag(","), space0), pair(tag(""), space1))), // alt needs all options to return the same type
                    alt((parse_hex_int32, parse_dec_int32)),
                ),
            ),
        ),
    )(input)
}

pub type DirectiveWordResult<'a> =
    IResult<&'a str, Vec<(Option<&'a str>, Result<i64, ParseIntError>)>>;

pub fn directive_word<'a>(input: &'a str) -> DirectiveWordResult {
    preceded(
        tag("."),
        preceded(
            tag("word"),
            preceded(
                space1,
                separated_nonempty_list(
                    alt((pair(tag(","), space0), pair(tag(""), space1))), // alt needs all options to return the same type
                    alt((parse_hex_int64, parse_dec_int64)),
                ),
            ),
        ),
    )(input)
}

pub type DirectiveSpaceResult<'a> = IResult<&'a str, (Option<&'a str>, Result<i64, ParseIntError>)>;

pub fn directive_space<'a>(input: &'a str) -> DirectiveSpaceResult {
    preceded(
        tag("."),
        preceded(
            tag("space"),
            preceded(space1, alt((parse_hex_int64, parse_dec_int64))),
        ),
    )(input)
}

pub type DirectiveKDataResult<'a> =
    IResult<&'a str, Option<(Option<&'a str>, Result<i64, ParseIntError>)>>;

pub fn directive_kdata<'a>(input: &'a str) -> DirectiveKDataResult {
    preceded(
        tag("."),
        preceded(
            tag("kdata"),
            opt(preceded(space1, alt((parse_hex_int64, parse_dec_int64)))),
        ),
    )(input)
}

pub type DirectiveKTextResult<'a> =
    IResult<&'a str, Option<(Option<&'a str>, Result<i64, ParseIntError>)>>;

pub fn directive_ktext<'a>(input: &'a str) -> DirectiveKTextResult {
    preceded(
        tag("."),
        preceded(
            tag("ktext"),
            opt(preceded(space1, alt((parse_hex_int64, parse_dec_int64)))),
        ),
    )(input)
}

pub type DirectiveTextResult<'a> =
    IResult<&'a str, Option<(Option<&'a str>, Result<i64, ParseIntError>)>>;

pub fn directive_text<'a>(input: &'a str) -> DirectiveTextResult {
    preceded(
        tag("."),
        preceded(
            tag("text"),
            opt(preceded(space1, alt((parse_hex_int64, parse_dec_int64)))),
        ),
    )(input)
}

pub enum ParsedDirective<'a> {
    Align(DirectiveAlignResult<'a>),
    Ascii(DirectiveAsciiResult<'a>),
    Asciiz(DirectiveAsciizResult<'a>),
    Byte(DirectiveByteResult<'a>),
    Data(DirectiveDataResult<'a>),
    Half(DirectiveHalfResult<'a>),
    KData(DirectiveKDataResult<'a>),
    KText(DirectiveKTextResult<'a>),
    Space(DirectiveSpaceResult<'a>),
    Text(DirectiveTextResult<'a>),
    Word(DirectiveWordResult<'a>),
}
