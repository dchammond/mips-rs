#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::fmt;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Debug)]
struct Label {
    pub addr: Option<u32>,
    pub label: String
}

impl Label {
    pub fn new<T,U>(addr: Option<T>, label: U) -> Label where u32: From<T>, String: From<U> {
        Label {addr: match addr { Some(a) => Some(a.into()), None => None }, label: label.into()}
    }
}

#[derive(Clone)]
struct State {
    pc: u32,
    registers: [u32; 32],
    memory: [u32; std::u16::MAX as usize],
    labels: Vec<Label>,
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "$pc: 0x{:08X} == {}\n", self.pc, self.pc)?;
        write!(f, "$0 : 0x{:08X} == {}\n", self.registers[0], self.registers[0])?;
        write!(f, "$at: 0x{:08X} == {}\n", self.registers[1], self.registers[1])?;
        write!(f, "$v0: 0x{:08X} == {}\n", self.registers[2], self.registers[2])?;
        write!(f, "$v1: 0x{:08X} == {}\n", self.registers[3], self.registers[3])?;
        write!(f, "$a0: 0x{:08X} == {}\n", self.registers[4], self.registers[4])?;
        write!(f, "$a1: 0x{:08X} == {}\n", self.registers[5], self.registers[5])?;
        write!(f, "$a2: 0x{:08X} == {}\n", self.registers[6], self.registers[6])?;
        write!(f, "$a3: 0x{:08X} == {}\n", self.registers[7], self.registers[7])?;
        write!(f, "$t0: 0x{:08X} == {}\n", self.registers[8], self.registers[8])?;
        write!(f, "$t1: 0x{:08X} == {}\n", self.registers[9], self.registers[9])?;
        write!(f, "$t2: 0x{:08X} == {}\n", self.registers[10], self.registers[10])?;
        write!(f, "$t3: 0x{:08X} == {}\n", self.registers[11], self.registers[11])?;
        write!(f, "$t4: 0x{:08X} == {}\n", self.registers[12], self.registers[12])?;
        write!(f, "$t5: 0x{:08X} == {}\n", self.registers[13], self.registers[13])?;
        write!(f, "$t6: 0x{:08X} == {}\n", self.registers[14], self.registers[14])?;
        write!(f, "$t7: 0x{:08X} == {}\n", self.registers[15], self.registers[15])?;
        write!(f, "$s0; 0x{:08X} == {}\n", self.registers[16], self.registers[16])?;
        write!(f, "$s1: 0x{:08X} == {}\n", self.registers[17], self.registers[17])?;
        write!(f, "$s2: 0x{:08X} == {}\n", self.registers[18], self.registers[18])?;
        write!(f, "$s3: 0x{:08X} == {}\n", self.registers[19], self.registers[19])?;
        write!(f, "$s4: 0x{:08X} == {}\n", self.registers[20], self.registers[20])?;
        write!(f, "$s5: 0x{:08X} == {}\n", self.registers[21], self.registers[21])?;
        write!(f, "$s6: 0x{:08X} == {}\n", self.registers[22], self.registers[22])?;
        write!(f, "$s7: 0x{:08X} == {}\n", self.registers[23], self.registers[23])?;
        write!(f, "$t8: 0x{:08X} == {}\n", self.registers[24], self.registers[24])?;
        write!(f, "$t9: 0x{:08X} == {}\n", self.registers[25], self.registers[25])?;
        write!(f, "$sp: 0x{:08X} == {}\n", self.registers[29], self.registers[29])?;
        write!(f, "$fp: 0x{:08X} == {}\n", self.registers[30], self.registers[30])?;
        write!(f, "$ra: 0x{:08X} == {}", self.registers[31], self.registers[31])
    }
}

#[derive(Clone)]
enum InstType {
    R(RType),
    I(IType),
    J(JType)
}

impl State {
    pub fn new() -> Self {
        State {pc: 0, registers: [0; 32], memory: [0; std::u16::MAX as usize], labels: Vec::new() }
    }
    pub fn run(&mut self) {
        /*
         * main's return address will be 0x0, if we ever jump here the program is done
         * 1. Set pc to 0x0 + 4
         * 2. Set $ra to 0x0 (which it already is)
         * 3. Begin executing code from memory
         */
        self.pc = 0;
        loop {
            self.pc += 4;
            match State::parse_instruction(self.memory[self.pc as usize]) {
                InstType::R(r) => {
                    r.perform(self);
                },
                InstType::I(i) => {
                    i.perform(self);
                },
                InstType::J(j) => {
                    j.perform(self);
                }
            }
            if self.pc == 0 {
                break;
            }
        }
    }
    pub fn parse_instruction<T>(inst: T) -> InstType where u32: From<T> {
        let inst: u32 = inst.into();
        let opcode: u32 = inst;
        let opcode: u32 = opcode >> 26;
        if opcode == 0 {
            InstType::R(RType::from(inst))
        } else if opcode == JInst::j.into() || opcode == JInst::jal.into()  {
            InstType::J(JType::from(inst))
        } else {
            InstType::I(IType::from(inst))
        }
    }
    pub fn load_compiled_instructions<T>(&mut self, instructions: &[u32], start: Option<T>) where u32: From<T> {
        let mut start: u32 = match start { Some(s) => s.into(), None => 0x4 };
        for inst in instructions {
            self.memory[start as usize] = *inst;
            start += 4;
        }
    }
    pub fn load_parsed_instructions<T>(&mut self, instructions: &[InstType], start: Option<T>) where u32: From<T> {
        let mut start: u32 = match start { Some(s) => s.into(), None => 0x4 };
        for inst in instructions {
            self.memory[start as usize] = match *inst {
                InstType::R(r) => r.into(),
                InstType::I(i) => i.into(),
                InstType::J(j) => j.into(),
            };
            start += 4;
        }
    }
    pub fn load_text_instructions<T>(&mut self, instructions: &[&str], start: Option<T>) where u32: From<T> {
        lazy_static! {
            static ref LABEL_RE: Regex = Regex::new(r"^\s*(?P<label>\w+):\s*$").unwrap();
        }
        let mut start: u32 = match start { Some(s) => s.into(), None => 0 };
        let mut labels: Vec<u32> = Vec::new();
        {
            let mut count = start;
            for line in instructions {
                let trim = line.trim();
                if trim == "" {
                    continue;
                }
                for caps in LABEL_RE.captures_iter(trim) {
                    self.add_label::<u32,&str>(Some(count), &caps["label"]);
                    labels.push(count);
                }
                count += 4;
            }
        }
        let mut iter = labels.into_iter().peekable();
        for inst in instructions {
            let inst = inst.trim();
            if inst == "" {
                continue;
            }
            if let Some(&i) = iter.peek() {
                if i == start {
                    iter.next();
                    start += 4;
                    continue;
                }
            }
            if let Some(r) = RType::convert_from_string(inst, &self) {
                self.memory[start as usize] = r.into();
            } else if let Some(i) = IType::convert_from_string(inst, &self) {
                self.memory[start as usize] = i.into();
            } else if let Some(j) = JType::convert_from_string(inst, &self) {
                self.memory[start as usize] = j.into();
            } else {
                panic!("Could not parse instruction: {}", inst);
            }
            start += 4;
        }
    }
    pub fn read_reg<T>(&self, r: T) -> u32 where u8: From<T> {
        self.registers[u8::from(r) as usize]
    }
    pub fn write_reg<T,U>(&mut self, r: T, val: U) where u8: From<T>, u32: From<U> {
        let reg = u8::from(r);
        match reg {
            0 => (),
            _ => self.registers[reg as usize] = u32::from(val),
        };
    }
    pub fn jump<T>(&mut self, dest: T) where u32: From<T> {
        self.pc = u32::from(dest);
    }
    pub fn write_mem<T, U>(&mut self, addr: T, val: U) where u32: From<T> + From<U> {
        self.memory[u32::from(addr) as usize] = u32::from(val);
    }
    pub fn read_mem<T>(&self, addr: T) -> u32 where u32: From<T> {
        self.memory[u32::from(addr) as usize]
    }
    pub fn find_label_by_addr<T>(&self, addr: T) -> Option<String> where u32: From<T> {
        let x = u32::from(addr);
        for p in &self.labels {
            match p.addr {
                Some(a) => if a == x { return Some(p.label.clone()); }
                None => (),
            }
        }
        None
    }
    pub fn find_label_by_name<T>(&self, name: T) -> Option<u32> where String: From<T> {
        let x = String::from(name);
        for p in &self.labels {
            if p.label == x {
                return p.addr;
            }
        }
        None
    }
    pub fn add_label<T,U>(&mut self, addr: Option<T>, label: U) where u32: From<T>, String: From<U>, U: Clone {
        let addr: Option<u32> = match addr { Some(a) => Some(a.into()), None => None };
        let label = String::from(label);
        for p in &mut self.labels {
            if p.label == label {
                match p.addr {
                    Some(_) => {return;},
                    None => {
                        p.addr = addr;
                        return;
                    }
                }
            }
        }
        self.labels.push(Label::new::<u32,String>(addr, label))
    }
}

#[derive(Copy, Clone, Debug)]
enum Reg {
    zero,
    at,
    v0, v1,
    a0, a1, a2, a3,
    t0, t1, t2, t3, t4, t5, t6, t7, t8, t9,
    s0, s1, s2, s3, s4, s5, s6, s7,
    sp,
    fp,
    ra
}

#[derive(Copy, Clone, Debug)]
enum Imm {
    Raw(u32),
    Label(u32),
    Address(u32),
}

macro_rules! imm_map {
    ($type_name: ty) => (
        impl From<$type_name> for Imm {
            fn from(n: $type_name) -> Imm {
                Imm::Raw(n as u32)
            }
        }
    );
}

imm_map!(u8);
imm_map!(u16);
imm_map!(u32);
imm_map!(u64);
imm_map!(u128);

macro_rules! imm_inv_map {
    ($type_name: ty) => (
        impl From<Imm> for $type_name {
            fn from(i: Imm) -> $type_name {
                match i {
                    Imm::Raw(r) => r as $type_name,
                    Imm::Label(l) => l as $type_name,
                    Imm::Address(a) => a as $type_name,
                }
            }
        }
    );
}

imm_inv_map!(i32);
imm_inv_map!(i64);
imm_inv_map!(i128);
imm_inv_map!(u16);
imm_inv_map!(u32);
imm_inv_map!(u64);
imm_inv_map!(u128);

impl From<Imm> for String {
    fn from(i: Imm) -> String {
        match i {
            Imm::Raw(r) => format!("0x{:08X}", r),
            Imm::Label(l) => format!("0x{:08X}", l),
            Imm::Address(a) => format!("0x{:08X}", a),
        }
    }
}

impl From<&str> for Reg {
    fn from(s: &str) -> Reg {
        match s.to_lowercase().as_ref() {
            "$zero" | "$0" => Reg::zero,
            "$at" | "$1" => Reg::at,
            "$v0" | "$2" => Reg::v0, "$v1" | "$3" => Reg::v1,
            "$a0" | "$4" => Reg::a0, "$a1" | "$5"=> Reg::a1, "$a2" | "$6" => Reg::a2, "$a3" | "$7" => Reg::a3, 
            "$t0" | "$8" => Reg::t0, "$t1" | "$9" => Reg::t1, "$t2" | "$10" => Reg::t2, "$t3" | "$11" => Reg::t3, "$t4" | "$12" => Reg::t4, "$t5" | "$13" => Reg::t5, "$t6" | "$14" => Reg::t6, "$t7" | "$15" => Reg::t7, "$t8" | "$24" => Reg::t8, "$t9" | "$25" => Reg::t9,
            "$s0" | "$16" => Reg::s0, "$s1" | "$17" => Reg::s1, "$s2" | "$18" => Reg::s2, "$s3" | "$19" => Reg::s3, "$s4" | "$20" => Reg::s4, "$s5" | "$21" => Reg::s5, "$s6" | "$22" => Reg::s6, "$s7" | "$23" => Reg::s7,
            "$sp" | "$29" => Reg::sp,
            "$fp" | "$30" => Reg::fp,
            "$ra" | "$31" => Reg::ra,
            _ => panic!("No such register: {}", s),
        }
    }
}

impl From<Reg> for String {
    fn from(r: Reg) -> String {
        match r {
            Reg::zero => "$zero",
            Reg::at => "$at",
            Reg::v0 => "$v0", Reg::v1 => "$v1",
            Reg::a0 => "$a0", Reg::a1 => "$a1", Reg::a2 => "$a2", Reg::a3 => "$a3", 
            Reg::t0 => "$t0", Reg::t1 => "$t1", Reg::t2 => "$t2", Reg::t3 => "$t3", Reg::t4 => "$t4", Reg::t5 => "$t5", Reg::t6 => "$t6", Reg::t7 => "$t7", Reg::t8 => "$t8", Reg::t9 => "$t9",
            Reg::s0 => "$s0", Reg::s1 => "$s1", Reg::s2 => "$s2", Reg::s3 => "$s3", Reg::s4 => "$s4", Reg::s5 => "$s5", Reg::s6 => "$s6", Reg::s7 => "$s7",
            Reg::sp => "$sp",
            Reg::fp => "$fp",
            Reg::ra => "$ra",
        }.to_owned()
    }
}

macro_rules! reg_map {
    ($type_name: ty) => (
        impl From<$type_name> for Reg {
            fn from(num: $type_name) -> Self {
                let num = (num as u8) & 0x1F;
                match num {
                    0  => Reg::zero,
                    1  => Reg::at,
                    2  => Reg::v0,
                    3  => Reg::v1,
                    4  => Reg::a0,
                    5  => Reg::a1,
                    6  => Reg::a2,
                    7  => Reg::a3,
                    8  => Reg::t0,
                    9  => Reg::t1,
                    10 => Reg::t2,
                    11 => Reg::t3,
                    12 => Reg::t4,
                    13 => Reg::t5,
                    14 => Reg::t6,
                    15 => Reg::t7,
                    16 => Reg::s0,
                    17 => Reg::s1,
                    18 => Reg::s2,
                    19 => Reg::s3,
                    20 => Reg::s4,
                    21 => Reg::s5,
                    22 => Reg::s6,
                    23 => Reg::s7,
                    24 => Reg::t8,
                    25 => Reg::t9,
                    29 => Reg::sp,
                    30 => Reg::fp,
                    31 => Reg::ra,
                    _  => unreachable!(),
                }
            }
        }
    );
}

macro_rules! reg_inv_map {
    ($type_name: ty) => (
        impl From<Reg> for $type_name {
            fn from(r: Reg) -> Self {
                match r {
                    Reg::zero => 0,
                    Reg::at => 1 ,
                    Reg::v0 => 2 ,
                    Reg::v1 => 3 ,
                    Reg::a0 => 4 ,
                    Reg::a1 => 5 ,
                    Reg::a2 => 6 ,
                    Reg::a3 => 7 ,
                    Reg::t0 => 8 ,
                    Reg::t1 => 9 ,
                    Reg::t2 => 10,
                    Reg::t3 => 11,
                    Reg::t4 => 12,
                    Reg::t5 => 13,
                    Reg::t6 => 14,
                    Reg::t7 => 15,
                    Reg::s0 => 16,
                    Reg::s1 => 17,
                    Reg::s2 => 18,
                    Reg::s3 => 19,
                    Reg::s4 => 20,
                    Reg::s5 => 21,
                    Reg::s6 => 22,
                    Reg::s7 => 23,
                    Reg::t8 => 24,
                    Reg::t9 => 25,
                    Reg::sp => 29,
                    Reg::fp => 30,
                    Reg::ra => 31,
                }
            }
        }
    );
}

reg_map!(u8);
reg_map!(u16);
reg_map!(u32);
reg_map!(u64);
reg_map!(u128);
reg_map!(i8);
reg_map!(i16);
reg_map!(i32);
reg_map!(i64);
reg_map!(i128);
reg_inv_map!(u8);
reg_inv_map!(u16);
reg_inv_map!(u32);
reg_inv_map!(u64);
reg_inv_map!(u128);
reg_inv_map!(i8);
reg_inv_map!(i16);
reg_inv_map!(i32);
reg_inv_map!(i64);
reg_inv_map!(i128);

#[derive(Copy, Clone, Debug)]
enum RInst {
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

#[derive(Copy, Clone, Debug)]
enum IInst {
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
            IInst::sw => "sw"
        }.to_owned()
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
    ($type_name: ty) => (
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
                    _    => panic!("No match for IType op-code: 0x{:08X}", num),
                }
            }
        }
    );
}

macro_rules! iinst_inv_map {
    ($type_name: ty) => (
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
    );
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

#[derive(Copy, Clone, Debug)]
enum JInst {
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

#[derive(Copy, Clone, Debug)]
struct RType {
    rs: Reg,
    rt: Reg,
    rd: Reg,
    shamt: u8,
    funct: RInst,
}

#[derive(Copy, Clone, Debug)]
struct IType {
    opcode: IInst,
    rs: Reg,
    rt: Reg,
    imm: Imm,
}

#[derive(Copy, Clone, Debug)]
struct JType {
    opcode: JInst,
    address: Imm,
}

impl RType {
    pub fn new<X,T,U,W,Q>(funct: X, rs: T, rt: U, rd: Q, shamt: W) -> RType where RInst: From<X>, Reg: From<T> + From<U> + From<Q>, u8: From<W> {
        RType {rs: Reg::from(rs), rt: Reg::from(rt), rd: Reg::from(rd), shamt: u8::from(shamt), funct: funct.into()}
    }
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
    pub fn convert_to_string(&self, state: &State) -> String {
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
    pub fn convert_from_string(string: &str, state: &State) -> Option<RType> {
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

impl IType {
    pub fn new<W,T,U,Q>(opcode: W, rs: T, rt: U, imm: Q) -> IType where IInst: From<W>, Reg: From<T> + From<U>, Imm: From<Q> {
        IType {opcode: opcode.into(), rs: Reg::from(rs), rt: Reg::from(rt), imm: Imm::from(imm)}
    }
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
            Imm::Raw(r) => None,
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
}

impl From<u32> for IType {
    fn from(n: u32) -> IType {
        let opcode = IInst::from(n >> 26);
        let rs = Reg::from(n >> 21);
        let rt = Reg::from(n >> 16);
        let imm = Imm::from(n);
        IType::new(opcode, rs, rt, imm)
    }
}

impl From<IType> for u32 {
    fn from(i: IType) -> u32 {
        let mut x = 0u32;
        x |= u32::from(i.opcode) << 26;
        x |= u32::from(i.rs) << 21;
        x |= u32::from(i.rt) << 16;
        x |= u32::from(i.imm);
        x
    }
}

impl JType {
    pub fn new<W,T>(opcode: W, address: T) -> JType where JInst: From<W>, u32: From<T> {
        JType {opcode: opcode.into(), address: Imm::Address(u32::from(address))}
    }
    pub fn perform(&self, state: &mut State) {
        let address = u32::from(self.address);
        match self.opcode {
            JInst::j => state.jump(address),
            JInst::jal => { state.write_reg(Reg::ra, state.pc); state.jump(address); },
        }
    }
    pub fn convert_to_string(&self, state: &State) -> String {
        let address_str_label = match self.address {
            Imm::Address(a) => state.find_label_by_addr(a),
            Imm::Label(l) => state.find_label_by_addr(l),
            Imm::Raw(r) => None,
        };
        let address_str = format!("0x{:08X}", u32::from(self.address));
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
}

impl From<u32> for JType {
    fn from(n: u32) -> JType {
        let opcode = JInst::from(n >> 26);
        let addr = Imm::Address(n & 0x3ffffff);
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

use std::{env, fs, io, path};
use std::io::Read;

fn load_file(state: &mut State, p: &path::Path, offset: Option<u32>) {
    lazy_static! {
        static ref COMMENT_RE: Regex = Regex::new(r"\s*#.*\s*\n").unwrap();
        static ref LABEL_CODE_RE: Regex = Regex::new(r"\s*(?P<label>\w+:)[^\n](?P<code>.+)").unwrap();
    }
    let mut file: fs::File;
    {
        let r: io::Result<fs::File> = fs::File::open(p);
        file = r.unwrap();
    }
    let mut file_contents: String = String::new();
    file.read_to_string(&mut file_contents).unwrap();
    file_contents = COMMENT_RE.replace_all(&file_contents, "\n").to_string();
    file_contents = LABEL_CODE_RE.replace_all(&file_contents, "\n$label\n$code").to_string(); // dumb hack to put label on its own line
    let file_contents = file_contents.split("\n");
    let file_contents: Vec<&str> = file_contents.collect();
    state.load_text_instructions(&file_contents[..], offset);
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <mips_file>", args[0]);
        return;
    }
    let mut state = State::new();
    load_file(&mut state, path::Path::new(&args[1]), None);
    state.run();
    println!("registers:\n{:?}", state);
}

