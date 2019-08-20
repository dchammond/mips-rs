//use crate::machine::state::State;

use crate::machine::immediate::Imm;

#[derive(Copy, Clone, Debug)]
pub struct JType {
    opcode: JInst,
    address: Imm,
}

impl JType {
    pub fn new(opcode: JInst, address: Imm) -> JType {
        JType {opcode, address}
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

impl From<u32> for JType {
    fn from(n: u32) -> JType {
        let opcode = JInst::from(n >> 26);
        let addr = Imm::Address(n & 0x3FF_FFFF);
        JType::new(opcode, addr)
    }
}

impl From<JType> for u32 {
    fn from(j: JType) -> u32 {
        let mut x = 0u32;
        x |= u32::from(j.opcode) << 26;
        x |= u32::from(j.address);
        x
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum JInst {
    j,
    jal,
}

impl From<JInst> for String {
    fn from(j: JInst) -> String {
        match j {
            JInst::j => "j",
            JInst::jal => "jal",
        }.to_owned()
    }
}

impl From<&str> for JInst {
    fn from(s: &str) -> JInst {
        match s.to_lowercase().as_ref() {
            "j" => JInst::j,
            "jal" => JInst::jal,
            _ => panic!("No such JType: {}", s)
        }
    }
}

macro_rules! jinst_map {
    ($type_name: ty) => (
        impl From<$type_name> for JInst {
            fn from(num: $type_name) -> Self {
                match num & 0x3F {
                    0x02 => JInst::j,
                    0x03 => JInst::jal,
                    _    => panic!("No match for JType op-code: 0x{:08X}", num),
                }
            }
        }
    );
}

macro_rules! jinst_inv_map {
    ($type_name: ty) => (
        impl From<JInst> for $type_name {
            fn from(j: JInst) -> Self {
                match j {
                    JInst::j => 0x02,
                    JInst::jal => 0x03,
                }
            }
        }
    );
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

