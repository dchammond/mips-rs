//use crate::machine::state::State;
use std::{convert::TryFrom, num::NonZeroU32};

use crate::machine::{address::Address, register::Reg};

#[derive(Clone, Debug)]
pub struct ITypeImm {
    pub opcode: IInst,
    pub rs: Reg,
    pub rt: Reg,
    pub imm: u16,
}

#[derive(Clone, Debug)]
pub struct ITypeLabel {
    pub opcode: IInst,
    pub rs: Reg,
    pub rt: Reg,
    pub label: Address,
}

impl ITypeImm {
    pub fn new(opcode: IInst, rs: Reg, rt: Reg, imm: u16) -> ITypeImm {
        ITypeImm {
            opcode,
            rs,
            rt,
            imm,
        }
    }
    /*
    pub fn perform(&self, state: &mut State) {
        let rs = state.read_reg(self.rs);
        let rt = state.read_reg(self.rt);
        let imm = u32::from(self.imm);
        match self.opcode {
            IInst::addi => state.write_reg(self.rt, i32::wrapping_add(rs as i32, imm as i32) as u32),
            IInst::addiu => state.write_reg(self.rt, u32::wrapping_add(rs, imm)),
            IInst::andi => state.write_reg(self.rt, rs & imm),
            IInst::beq => if rs == rt { state.jump(imm) },
            IInst::bne => if rs != rt { state.jump(imm) },
            IInst::lbu => state.write_reg(self.rt, state.read_mem(u32::wrapping_add(rs, imm)) & 0xFFu32),
            IInst::lhu => state.write_reg(self.rt, state.read_mem(u32::wrapping_add(rs, imm)) & 0xFFFFu32),
            IInst::ll | IInst::lw => state.write_reg(self.rt, state.read_mem(u32::wrapping_add(rs, imm))),
            IInst::li | IInst::la => state.write_reg(self.rt, imm),
            IInst::lui => state.write_reg(self.rt, imm << 16),
            IInst::ori => state.write_reg(self.rt, rs | imm),
            IInst::slti => state.write_reg(self.rt, match (rs as i32) < (imm as i32) { true => 1u32, false => 0u32 }),
            IInst::sltiu => state.write_reg(self.rt, match rs < imm { true => 1u32, false => 0u32 }),
            IInst::sb => state.write_mem(u32::wrapping_add(rs, imm), rt & 0xFFu32),
            IInst::sc => unimplemented!(),
            IInst::sh => state.write_mem(u32::wrapping_add(rs, imm), rt & 0xFFFFu32),
            IInst::sw => state.write_mem(u32::wrapping_add(rs, imm), rt),
        }
    }
    pub fn convert_to_string(&self, state: &State) -> String {
        let imm_str_label = match self.imm {
            Imm::Address(j) => state.find_label_by_addr(j),
            Imm::Label(l) => state.find_label_by_addr(l),
            Imm::Raw(_) => None,
        };
        let imm_str = format!("0x{:08X}", u16::from(self.imm));
        match self.opcode {
            IInst::addi  |
                IInst::addiu |
                IInst::andi  |
                IInst::ori   |
                IInst::slti  |
                IInst::sltiu => {
                    format!("{} {}, {}, {}", String::from(self.opcode), String::from(self.rt), String::from(self.rs), imm_str)
                },
                IInst::beq   |
                IInst::bne => {
                    let branch_imm = match imm_str_label {
                        Some(s) => s,
                        None => {
                            state.find_label_by_addr(u16::from(self.imm)).unwrap()
                        }
                    };
                    format!("{} {}, {}, {}", String::from(self.opcode), String::from(self.rt), String::from(self.rs), branch_imm)
                },
                IInst::lbu |
                IInst::lhu |
                IInst::ll  |
                IInst::lw  |
                IInst::sb  |
                IInst::sh  |
                IInst::sw  => {
                    format!("{} {}, {}({})", String::from(self.opcode), String::from(self.rt), imm_str, String::from(self.rs))
                },
                IInst::li  |
                IInst::lui |
                IInst::la => {
                    format!("{} {}, {}", String::from(self.opcode), String::from(self.rt), imm_str)
                },
                IInst::sc => unimplemented!()
        }
    }
    pub fn convert_from_string(string: &str, state: &State) -> Option<IType> {
        lazy_static! {
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
        for caps in I_ARITH_HEX_RE.captures_iter(string) {
            return Some(IType::new(&caps["opcode"], &caps["rs"], &caps["rt"], u32::from_str_radix(&caps["imm"], 16).unwrap()));
        }
        for caps in I_ARITH_DEC_RE.captures_iter(string) {
            return Some(IType::new(&caps["opcode"], &caps["rs"], &caps["rt"], u32::from_str_radix(&caps["imm"], 10).unwrap()));
        }
        for caps in I_BRANCH_HEX_RE.captures_iter(string) {
            return Some(IType::new(&caps["opcode"], &caps["rs"], &caps["rt"], Imm::Raw(u32::from_str_radix(&caps["imm"], 16).unwrap())));
        }
        for caps in I_BRANCH_STR_RE.captures_iter(string) {
            match state.find_label_by_name(&caps["label"]) {
                Some(a) => return Some(IType::new(&caps["opcode"], &caps["rs"], &caps["rt"], a)),
                None => {
                    panic!("Unresolved label: {}", string);
                }
            }
        }
        for caps in I_MEM_HEX_RE.captures_iter(string) {
            return Some(IType::new(&caps["opcode"], &caps["rs"], &caps["rt"], Imm::Raw(u32::from_str_radix(&caps["imm"], 16).unwrap())));
        }
        for caps in I_MEM_DEC_RE.captures_iter(string) {
            return Some(IType::new(&caps["opcode"], &caps["rs"], &caps["rt"], Imm::Raw(u32::from_str_radix(&caps["imm"], 10).unwrap())));
        }
        for caps in I_MEM_STR_RE.captures_iter(string) {
            match state.find_label_by_name(&caps["label"]) {
                Some(a) => return Some(IType::new(&caps["opcode"], &caps["rs"], &caps["rt"], a)),
                None => {
                    panic!("Unresolved label: {}", string);
                }
            }
        }
        for caps in I_IMM_HEX_RE.captures_iter(string) {
            return Some(IType::new(&caps["opcode"], 0u16, &caps["rt"], Imm::Raw(u32::from_str_radix(&caps["imm"], 16).unwrap())));
        }
        for caps in I_IMM_DEC_RE.captures_iter(string) {
            return Some(IType::new(&caps["opcode"], 0u16, &caps["rt"], Imm::Raw(u32::from_str_radix(&caps["imm"], 10).unwrap())));
        }
        for caps in I_IMM_STR_RE.captures_iter(string) {
            match state.find_label_by_name(&caps["label"]) {
                Some(a) => return Some(IType::new(&caps["opcode"], 0u16, &caps["rt"], a)),
                None => {
                    panic!("Unresolved label: {}", string);
                }
            }
        }
        None
    }
    */
}

impl ITypeLabel {
    pub fn new(opcode: IInst, rs: Reg, rt: Reg, label: Address) -> ITypeLabel {
        ITypeLabel {
            opcode,
            rs,
            rt,
            label,
        }
    }
}

impl From<u32> for ITypeImm {
    fn from(n: u32) -> ITypeImm {
        let opcode = IInst::from(n >> 26);
        let rs = Reg::from(n >> 21);
        let rt = Reg::from(n >> 16);
        let imm = (n & 0xFFFF) as u16;
        ITypeImm::new(opcode, rs, rt, imm)
    }
}

impl From<ITypeImm> for u32 {
    fn from(i: ITypeImm) -> Self {
        let mut x = 0u32;
        x |= u32::from(i.opcode) << 26;
        x |= u32::from(i.rs) << 21;
        x |= u32::from(i.rt) << 16;
        x |= u32::from(i.imm);
        x
    }
}

impl From<u32> for ITypeLabel {
    fn from(n: u32) -> Self {
        let opcode = IInst::from(n >> 26);
        let rs = Reg::from(n >> 21);
        let rt = Reg::from(n >> 16);
        let addr_raw = n & 0xFFFF;
        if addr_raw == 0 {
            panic!(
                "Cannot convert 0x{:08X} into ITypeLabel because immediate is 0",
                n
            );
        }
        let addr;
        unsafe {
            addr = Address::new(Some(NonZeroU32::new_unchecked(addr_raw)), None);
        }
        ITypeLabel::new(opcode, rs, rt, addr)
    }
}

impl TryFrom<ITypeLabel> for u32 {
    type Error = String;

    fn try_from(i: ITypeLabel) -> Result<Self, Self::Error> {
        match i.label.numeric {
            Some(nz) => {
                let mut x = 0u32;
                x |= u32::from(i.opcode) << 26;
                x |= u32::from(i.rs) << 21;
                x |= u32::from(i.rt) << 16;
                x |= nz.get() & 0xFFFF;
                Ok(x)
            }
            None => Err(format!(
                "Cannot convert ITypeLabel to u32, immediate is {:#?}",
                i.label
            )),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum IInst {
    addi,
    addiu,
    andi,
    beq,
    bne,
    lbu,
    lhu,
    ll,
    li,
    la,
    lui,
    lw,
    ori,
    slti,
    sltiu,
    sb,
    sc,
    sh,
    sw,
}

impl From<IInst> for String {
    fn from(i: IInst) -> String {
        match i {
            IInst::addi => "addi",
            IInst::addiu => "addiu",
            IInst::andi => "andi",
            IInst::beq => "beq",
            IInst::bne => "bne",
            IInst::lbu => "lbu",
            IInst::lhu => "lhu",
            IInst::ll => "ll",
            IInst::li => "li",
            IInst::la => "la",
            IInst::lui => "lui",
            IInst::lw => "lw",
            IInst::ori => "ori",
            IInst::slti => "slti",
            IInst::sltiu => "sltiu",
            IInst::sb => "sb",
            IInst::sc => "sc",
            IInst::sh => "sh",
            IInst::sw => "sw",
        }
        .to_owned()
    }
}

impl From<&str> for IInst {
    fn from(s: &str) -> IInst {
        match s.to_lowercase().as_ref() {
            "addi" => IInst::addi,
            "addiu" => IInst::addiu,
            "andi" => IInst::andi,
            "beq" => IInst::beq,
            "bne" => IInst::bne,
            "lbu" => IInst::lbu,
            "lhu" => IInst::lhu,
            "ll" => IInst::ll,
            "li" => IInst::li,
            "la" => IInst::la,
            "lui" => IInst::lui,
            "lw" => IInst::lw,
            "ori" => IInst::ori,
            "slti" => IInst::slti,
            "sltiu" => IInst::sltiu,
            "sb" => IInst::sb,
            "sc" => IInst::sc,
            "sh" => IInst::sh,
            "sw" => IInst::sw,
            _ => panic!("No such IType: {}", s),
        }
    }
}

macro_rules! iinst_map {
    ($type_name: ty) => {
        impl From<$type_name> for IInst {
            fn from(num: $type_name) -> Self {
                match num & 0x3F {
                    0x08 => IInst::addi,
                    0x09 => IInst::addiu,
                    0x0C => IInst::andi,
                    0x04 => IInst::beq,
                    0x05 => IInst::bne,
                    0x24 => IInst::lbu,
                    0x25 => IInst::lhu,
                    0x30 => IInst::ll,
                    0x3F => IInst::li,
                    0x01 => IInst::la,
                    0x0F => IInst::lui,
                    0x23 => IInst::lw,
                    0x0D => IInst::ori,
                    0x0A => IInst::slti,
                    0x0B => IInst::sltiu,
                    0x28 => IInst::sb,
                    0x38 => IInst::sc,
                    0x29 => IInst::sh,
                    0x2B => IInst::sw,
                    _ => panic!("No match for IType op-code: 0x{:08X}", num),
                }
            }
        }
    };
}

macro_rules! iinst_inv_map {
    ($type_name: ty) => {
        impl From<IInst> for $type_name {
            fn from(i: IInst) -> Self {
                match i {
                    IInst::addi => 0x08,
                    IInst::addiu => 0x09,
                    IInst::andi => 0x0C,
                    IInst::beq => 0x04,
                    IInst::bne => 0x05,
                    IInst::lbu => 0x24,
                    IInst::lhu => 0x25,
                    IInst::ll => 0x30,
                    IInst::li => 0x3F,
                    IInst::la => 0x01,
                    IInst::lui => 0x0F,
                    IInst::lw => 0x23,
                    IInst::ori => 0x0D,
                    IInst::slti => 0x0A,
                    IInst::sltiu => 0x0B,
                    IInst::sb => 0x28,
                    IInst::sc => 0x38,
                    IInst::sh => 0x29,
                    IInst::sw => 0x2B,
                }
            }
        }
    };
}

iinst_map!(u8);
iinst_map!(u16);
iinst_map!(u32);
iinst_map!(u64);
iinst_map!(u128);
iinst_map!(i8);
iinst_map!(i16);
iinst_map!(i32);
iinst_map!(i64);
iinst_map!(i128);
iinst_inv_map!(u8);
iinst_inv_map!(u16);
iinst_inv_map!(u32);
iinst_inv_map!(u64);
iinst_inv_map!(u128);
iinst_inv_map!(i8);
iinst_inv_map!(i16);
iinst_inv_map!(i32);
iinst_inv_map!(i64);
iinst_inv_map!(i128);
