//use crate::machine::state::State;
use crate::machine::register::Reg;

#[derive(Copy, Clone, Debug)]
pub struct RType {
    rs: Reg,
    rt: Reg,
    rd: Reg,
    shamt: u8,
    funct: RInst,
}

impl RType {
    pub fn new(funct: RInst, rs: Reg, rt: Reg, rd: Reg, shamt: u8) -> RType {
        RType {rs, rt, rd, shamt, funct}
    }
    /*
    pub fn perform(&self, state: &mut State) {
        let rs = state.read_reg(self.rs);
        let rt = state.read_reg(self.rt);
        match self.funct {
            RInst::add => state.write_reg(self.rd, i32::wrapping_add(rs as i32, rt as i32) as u32),
            RInst::addu => state.write_reg(self.rd, u32::wrapping_add(rs, rt)),
            RInst::and => state.write_reg(self.rd, rs & rt),
            RInst::jr => state.jump(rs),
            RInst::nor => state.write_reg(self.rd, !(rs | rt)),
            RInst::or => state.write_reg(self.rd, rs | rt),
            RInst::slt => state.write_reg(self.rd, match (rs as i32) < (rt as i32) { true => 1u32, false => 0u32 }),
            RInst::sltu => state.write_reg(self.rd, match rs < rt { true => 1u32, false => 0u32 }),
            RInst::sll => state.write_reg(self.rd, rt << self.shamt),
            RInst::srl => state.write_reg(self.rd, rt >> self.shamt),
            RInst::sub => state.write_reg(self.rd, i32::wrapping_sub(rs as i32, rt as i32) as u32),
            RInst::subu => state.write_reg(self.rd, u32::wrapping_sub(rs, rt)),
            RInst::div => state.write_reg(self.rd, ((rs as i32) / (rt as i32)) as u32),
            RInst::divu => state.write_reg(self.rd, rs / rt),
        }
    }
    pub fn convert_to_string(&self, _state: &State) -> String {
        match self.funct {
            RInst::add  |
                RInst::addu |
                RInst::and  |
                RInst::nor  |
                RInst::or   |
                RInst::slt  |
                RInst::sltu |
                RInst::sub  |
                RInst::subu |
                RInst::div  |
                RInst::divu => {
                    format!("{} {}, {}, {}", String::from(self.funct), String::from(self.rd), String::from(self.rs), String::from(self.rt))
                },
                RInst::sll |
                    RInst::srl => {
                        format!("{} {}, {}, 0x{:08X}", String::from(self.funct), String::from(self.rd), String::from(self.rs), self.shamt)
                    },
                RInst::jr => {
                    format!("{} {}", String::from(self.funct), String::from(self.rs))
                },
        }
    }
    
    pub fn convert_from_string(string: &str, _state: &State) -> Option<RType> {
        lazy_static! {
            static ref R_ARITH_RE: Regex = Regex::new(r"^\s*(?P<funct>\w+)\s*(?P<rd>\$[\w\d]+),\s*(?P<rs>\$[\w\d]+),\s*(?P<rt>\$[\w\d]+)\s*$").unwrap();
            static ref R_SHIFT_HEX_RE: Regex = Regex::new(r"^\s*(?P<funct>\w+)\s*(?P<rd>\$[\w\d]+),\s*(?P<rs>\$[\w\d]+),\s*0x(?P<shamt>[\da-fA-F]+)\s*$").unwrap();
            static ref R_SHIFT_DEC_RE: Regex = Regex::new(r"^\s*(?P<funct>\w+)\s*(?P<rd>\$]\w\d]+),\s*(?P<rs>\$[\w\d]+),\s*(?P<shamt>\d+)\s*$").unwrap();
            static ref R_JUMP_RE: Regex = Regex::new(r"^\s*(?P<funct>\w+)\s*(?P<rs>\$[\w\d]+)\s*$").unwrap();
        }
        for caps in R_ARITH_RE.captures_iter(string) {
            return Some(RType::new(&caps["funct"], &caps["rs"], &caps["rt"], &caps["rd"], 0u8));
        }
        for caps in R_SHIFT_HEX_RE.captures_iter(string) {
            if &caps["funct"] != "sll" && &caps["funct"] != "srl" {
                continue;
            }
            return Some(RType::new(&caps["funct"], &caps["rs"], 0u8, &caps["rd"], u8::from_str_radix(&caps["shamt"], 16).unwrap()));
        }
        for caps in R_SHIFT_DEC_RE.captures_iter(string) {
            if &caps["funct"] != "sll" && &caps["funct"] != "srl" {
                continue;
            }
            return Some(RType::new(&caps["funct"], &caps["rs"], 0u8, &caps["rd"], u8::from_str_radix(&caps["shamt"], 10).unwrap()));
        }
        for caps in R_JUMP_RE.captures_iter(string) {
            return Some(RType::new(&caps["funct"], &caps["rs"], 0u8, 0u8, 0u8));
        }
        None
    }
    */
}

impl From<u32> for RType {
    fn from(n: u32) -> RType {
        let rs = Reg::from(n >> 21);
        let rt = Reg::from(n >> 16);
        let rd = Reg::from(n >> 11);
        let shamt = ((n >> 6) & 0x1F) as u8;
        let funct = RInst::from(n);
        RType::new(funct, rs, rt, rd, shamt)
    }
}

impl From<RType> for u32 {
    fn from(r: RType) -> u32 {
        let mut x = 0u32;
        x |= u32::from(r.rs) << 21;
        x |= u32::from(r.rt) << 16;
        x |= u32::from(r.rd) << 11;
        x |= (r.shamt as u32) << 6;
        x |= u32::from(r.funct);
        x
    }
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum RInst {
    add,
    addu,
    and,
    jr,
    nor,
    or,
    slt,
    sltu,
    sll,
    srl,
    sub,
    subu,
    div,
    divu,
}

impl From<RInst> for String {
    fn from(r: RInst) -> String {
        match r {
            RInst::add => "add",
            RInst::addu => "addu",
            RInst::and => "and",
            RInst::jr => "jr",
            RInst::nor => "nor",
            RInst::or => "or",
            RInst::slt => "slt",
            RInst::sltu => "sltu",
            RInst::sll => "sll",
            RInst::srl => "srl",
            RInst::sub => "sub",
            RInst::subu => "subu",
            RInst::div => "div",
            RInst::divu => "divu"
        }.to_owned()
    }
}

impl From<&str> for RInst {
    fn from(s: &str) -> RInst {
        match s.to_lowercase().as_ref() {
            "add" => RInst::add,
            "addu" => RInst::addu,
            "and" => RInst::and,
            "jr" => RInst::jr,
            "nor" =>  RInst::nor,
            "or" =>  RInst::or,
            "slt" => RInst::slt,
            "sltu" => RInst::sltu,
            "sll" => RInst::sll,
            "srl" => RInst::srl,
            "sub" => RInst::sub,
            "subu" => RInst::subu,
            "div" =>  RInst::div,
            "divu" => RInst::divu,
            _ => panic!("No match for RType: {}", s)
        }
    }
}

macro_rules! rinst_map {
    ($type_name: ty) => (
        impl From<$type_name> for RInst {
            fn from(num: $type_name) -> Self {
                match num & 0x3F {
                    0x20 => RInst::add,
                    0x21 => RInst::addu,
                    0x24 => RInst::and,
                    0x08 => RInst::jr,
                    0x27 => RInst::nor,
                    0x25 => RInst::or,
                    0x2A => RInst::slt,
                    0x2B => RInst::sltu,
                    0x00 => RInst::sll,
                    0x02 => RInst::srl,
                    0x22 => RInst::sub,
                    0x23 => RInst::subu,
                    0x1A => RInst::div,
                    0x1B => RInst::divu,
                    _    => panic!("No match for RType funct code: 0x{:08X}", num),
                }
            }
        }
    );
}

macro_rules! rinst_inv_map {
    ($type_name: ty) => (
        impl From<RInst> for $type_name {
            fn from(r: RInst) -> Self {
                match r {
                    RInst::add => 0x20,
                    RInst::addu => 0x21,
                    RInst::and => 0x24,
                    RInst::jr => 0x08,
                    RInst::nor => 0x27,
                    RInst::or => 0x25,
                    RInst::slt => 0x2A,
                    RInst::sltu => 0x2B,
                    RInst::sll => 0x00,
                    RInst::srl => 0x02,
                    RInst::sub => 0x22,
                    RInst::subu => 0x23,
                    RInst::div => 0x1A,
                    RInst::divu => 0x1B,
                }
            }
        }
    );
}

rinst_map!(u8);
rinst_map!(u16);
rinst_map!(u32);
rinst_map!(u64);
rinst_map!(u128);
rinst_map!(i8);
rinst_map!(i16);
rinst_map!(i32);
rinst_map!(i64);
rinst_map!(i128);
rinst_inv_map!(u8);
rinst_inv_map!(u16);
rinst_inv_map!(u32);
rinst_inv_map!(u64);
rinst_inv_map!(u128);
rinst_inv_map!(i8);
rinst_inv_map!(i16);
rinst_inv_map!(i32);
rinst_inv_map!(i64);
rinst_inv_map!(i128);

