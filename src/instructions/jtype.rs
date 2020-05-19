//use crate::machine::state::State;
use std::{convert::TryFrom, num::NonZeroU32};

use crate::machine::address::Address;

#[derive(Clone, Debug)]
pub struct JTypeImm {
    pub opcode: JInst,
    pub imm: u32,
}

#[derive(Clone, Debug)]
pub struct JTypeLabel {
    pub opcode: JInst,
    pub label: Address,
}

impl JTypeImm {
    pub fn new(opcode: JInst, imm: u32) -> JTypeImm {
        JTypeImm { opcode, imm }
    }
    /*
    pub fn perform(&self, state: &mut State) {
        let address = u32::from(self.address);
        match self.opcode {
            JInst::j => state.jump(address),
            JInst::jal => { state.write_reg(Reg::ra, state.read_pc()); state.jump(address); },
        }
    }
    pub fn convert_to_string(&self, state: &State) -> String {
        let address_str_label: Option<String> = match self.address {
            Imm::Address(a) => state.find_label_by_addr(a),
            Imm::Label(l) => state.find_label_by_addr(l),
            Imm::Raw(_) => None,
        };
        let address_str: String;
        if let Some(l) = address_str_label {
            address_str = l;
        } else {
            address_str = format!("0x{:08X}", u32::from(self.address));
        }
        match self.opcode {
            JInst::j | JInst::jal => format!("{} {}", String::from(self.opcode), address_str),
        }
    }
    pub fn convert_from_string(string: &str, state: &State) -> Option<JType> {
        lazy_static! {
            static ref J_STR_RE: Regex = Regex::new(r"^\s*(?P<opcode>\w+)\s*(?P<label>\w+)s*$").unwrap();
        }
        for caps in J_STR_RE.captures_iter(string) {
            match state.find_label_by_name(&caps["label"]) {
                Some(a) => return Some(JType::new(&caps["opcode"], a)),
                None => {
                    panic!("Unresolved label: {}", string);
                }
            }
        }
        None
    }
    */
}

impl JTypeLabel {
    pub fn new(opcode: JInst, label: Address) -> JTypeLabel {
        JTypeLabel { opcode, label }
    }
}

impl From<u32> for JTypeImm {
    fn from(n: u32) -> JTypeImm {
        let opcode = JInst::from(n >> 26);
        let addr_raw = n & 0x3FF_FFFF;
        if addr_raw == 0 {
            panic!("Cannot convert 0x{:08X} into JType because address is 0", n);
        }
        JTypeImm::new(opcode, addr_raw)
    }
}

impl From<JTypeImm> for u32 {
    fn from(j: JTypeImm) -> Self {
        let mut x = 0u32;
        x |= u32::from(j.opcode) << 26;
        x |= u32::from(j.imm);
        x
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum JInst {
    j,
    jal,
    eret,
}

impl From<JInst> for String {
    fn from(j_: JInst) -> String {
        match j_ {
            JInst::j => "j",
            JInst::jal => "jal",
            JInst::eret => "eret",
        }
        .to_owned()
    }
}

impl From<&str> for JInst {
    fn from(s: &str) -> JInst {
        match s.to_lowercase().as_ref() {
            "j" => JInst::j,
            "jal" => JInst::jal,
            "eret" => JInst::eret,
            _ => panic!("No such JType: {}", s),
        }
    }
}

macro_rules! jinst_map {
    ($type_name: ty) => {
        impl From<$type_name> for JInst {
            fn from(num: $type_name) -> Self {
                match num & 0x3F {
                    0x02 => JInst::j,
                    0x03 => JInst::jal,
                    0x18 => JInst::eret,
                    _ => panic!("No match for JType op-code: 0x{:08X}", num),
                }
            }
        }
    };
}

macro_rules! jinst_inv_map {
    ($type_name: ty) => {
        impl From<JInst> for $type_name {
            fn from(j: JInst) -> Self {
                match j {
                    JInst::j => 0x02,
                    JInst::jal => 0x03,
                    JInst::eret => 0x18,
                }
            }
        }
    };
}

jinst_map!(u8);
jinst_map!(u16);
jinst_map!(u32);
jinst_map!(u64);
jinst_map!(u128);
jinst_map!(i8);
jinst_map!(i16);
jinst_map!(i32);
jinst_map!(i64);
jinst_map!(i128);
jinst_inv_map!(u8);
jinst_inv_map!(u16);
jinst_inv_map!(u32);
jinst_inv_map!(u64);
jinst_inv_map!(u128);
jinst_inv_map!(i8);
jinst_inv_map!(i16);
jinst_inv_map!(i32);
jinst_inv_map!(i64);
jinst_inv_map!(i128);
