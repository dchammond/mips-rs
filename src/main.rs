#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::fmt;

#[derive(Copy, Clone)]
struct State {
    pc: u32,
    registers: [u32; 32],
    memory: [u32; std::u16::MAX as usize],
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "$pc: 0x{:x} == {}\n", self.pc, self.pc)?;
        write!(f, "$0 : 0x{:x} == {}\n", self.registers[0], self.registers[0])?;
        write!(f, "$at: 0x{:x} == {}\n", self.registers[1], self.registers[1])?;
        write!(f, "$v0: 0x{:x} == {}\n", self.registers[2], self.registers[2])?;
        write!(f, "$v1: 0x{:x} == {}\n", self.registers[3], self.registers[3])?;
        write!(f, "$a0: 0x{:x} == {}\n", self.registers[4], self.registers[4])?;
        write!(f, "$a1: 0x{:x} == {}\n", self.registers[5], self.registers[5])?;
        write!(f, "$a2: 0x{:x} == {}\n", self.registers[6], self.registers[6])?;
        write!(f, "$a3: 0x{:x} == {}\n", self.registers[7], self.registers[7])?;
        write!(f, "$t0: 0x{:x} == {}\n", self.registers[8], self.registers[8])?;
        write!(f, "$t1: 0x{:x} == {}\n", self.registers[9], self.registers[9])?;
        write!(f, "$t2: 0x{:x} == {}\n", self.registers[10], self.registers[10])?;
        write!(f, "$t3: 0x{:x} == {}\n", self.registers[11], self.registers[11])?;
        write!(f, "$t4: 0x{:x} == {}\n", self.registers[12], self.registers[12])?;
        write!(f, "$t5: 0x{:x} == {}\n", self.registers[13], self.registers[13])?;
        write!(f, "$t6: 0x{:x} == {}\n", self.registers[14], self.registers[14])?;
        write!(f, "$t7: 0x{:x} == {}\n", self.registers[15], self.registers[15])?;
        write!(f, "$s0; 0x{:x} == {}\n", self.registers[16], self.registers[16])?;
        write!(f, "$s1: 0x{:x} == {}\n", self.registers[17], self.registers[17])?;
        write!(f, "$s2: 0x{:x} == {}\n", self.registers[18], self.registers[18])?;
        write!(f, "$s3: 0x{:x} == {}\n", self.registers[19], self.registers[19])?;
        write!(f, "$s4: 0x{:x} == {}\n", self.registers[20], self.registers[20])?;
        write!(f, "$s5: 0x{:x} == {}\n", self.registers[21], self.registers[21])?;
        write!(f, "$s6: 0x{:x} == {}\n", self.registers[22], self.registers[22])?;
        write!(f, "$s7: 0x{:x} == {}\n", self.registers[23], self.registers[23])?;
        write!(f, "$t8: 0x{:x} == {}\n", self.registers[24], self.registers[24])?;
        write!(f, "$t9: 0x{:x} == {}\n", self.registers[25], self.registers[25])?;
        write!(f, "$sp: 0x{:x} == {}\n", self.registers[29], self.registers[29])?;
        write!(f, "$fp: 0x{:x} == {}\n", self.registers[30], self.registers[30])?;
        write!(f, "$ra: 0x{:x} == {}", self.registers[31], self.registers[31])
    }
}

impl State {
    pub fn new() -> Self {
        State {pc: 0, registers: [0; 32], memory: [0; std::u16::MAX as usize] }
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
    pub fn write_mem<T, U>(&mut self, addr: T, val: U) where u32: From<T> + From<U> {
        self.memory[u32::from(addr) as usize] = u32::from(val);
    }
    pub fn read_mem<T>(&self, addr: T) -> u32 where u32: From<T> {
        self.memory[u32::from(addr) as usize]
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

macro_rules! reg_map {
    ($type_name: ty) => (
        impl From<$type_name> for Reg {
            fn from(num: $type_name) -> Self {
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
                    _  => unimplemented!()
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
    Add,
    Addu,
    And,
    Jr,
    Nor,
    Or,
    Slt,
    Sltu,
    Sll,
    Srl,
    Sub,
    Subu,
    Div,
    Divu,
}

macro_rules! rinst_map {
    ($type_name: ty) => (
        impl From<$type_name> for RInst {
            fn from(num: $type_name) -> Self {
                match num {
                    0x20 => RInst::Add,
                    0x21 => RInst::Addu,
                    0x24 => RInst::And,
                    0x08 => RInst::Jr,
                    0x27 => RInst::Nor,
                    0x25 => RInst::Or,
                    0x2A => RInst::Slt,
                    0x2B => RInst::Sltu,
                    0x00 => RInst::Sll,
                    0x02 => RInst::Srl,
                    0x22 => RInst::Sub,
                    0x23 => RInst::Subu,
                    0x1A => RInst::Div,
                    0x1B => RInst::Divu,
                    _    => unimplemented!(),
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
                    RInst::Add => 0x20,
                    RInst::Addu => 0x21,
                    RInst::And => 0x24,
                    RInst::Jr => 0x08,
                    RInst::Nor => 0x27,
                    RInst::Or => 0x25,
                    RInst::Slt => 0x2A,
                    RInst::Sltu => 0x2B,
                    RInst::Sll => 0x00,
                    RInst::Srl => 0x02,
                    RInst::Sub => 0x22,
                    RInst::Subu => 0x23,
                    RInst::Div => 0x1A,
                    RInst::Divu => 0x1B,
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
    Addi,
    Addiu,
    Andi,
    Beq,
    Bne,
    Lbu,
    Lhu,
    Ll,
    Li,
    Lui,
    Lw,
    Ori,
    Slti,
    Sltiu,
    Sb,
    Sc,
    Sh,
    Sw,
}

impl Into<String> for IInst {
    fn into(self) -> String {
        match self {
           IInst::Addi => "addi",
           IInst::Addiu => "addiu",
           IInst::Andi => "andi",
           IInst::Beq => "beq",
           IInst::Bne => "bne",
           IInst::Lbu => "lbu",
           IInst::Lhu => "lhu",
           IInst::Ll => "ll",
           IInst::Li => "li",
           IInst::Lui => "lui",
           IInst::Lw => "lw",
           IInst::Ori => "ori",
           IInst::Slti => "slti",
           IInst::Sltiu => "sltiu",
           IInst::Sb => "sb",
           IInst::Sc => "sc",
           IInst::Sh => "sh",
           IInst::Sw => "sw"
        }.to_owned()
    }
}

impl Into<IInst> for &str {
    fn into(self) -> IInst {
        match self.to_lowercase().as_ref() {
           "addi" => IInst::Addi,
           "addiu" => IInst::Addiu,
           "andi" => IInst::Andi,
           "beq" => IInst::Beq,
           "bne" => IInst::Bne,
           "lbu" => IInst::Lbu,
           "lhu" => IInst::Lhu,
           "ll" => IInst::Ll,
           "li" => IInst::Li,
           "lui" => IInst::Lui,
           "lw" => IInst::Lw,
           "ori" => IInst::Ori,
           "slti" => IInst::Slti,
           "sltiu" => IInst::Sltiu,
           "sb" => IInst::Sb,
           "sc" => IInst::Sc,
           "sh" => IInst::Sh,
           "sw" => IInst::Sw,
           _ => unimplemented!(),
        }
    }
}

macro_rules! iinst_map {
    ($type_name: ty) => (
        impl From<$type_name> for IInst {
            fn from(num: $type_name) -> Self {
                match num {
                    0x08 => IInst::Addi,
                    0x09 => IInst::Addiu,
                    0x0C => IInst::Andi,
                    0x04 => IInst::Beq,
                    0x05 => IInst::Bne,
                    0x24 => IInst::Lbu,
                    0x25 => IInst::Lhu,
                    0x30 => IInst::Ll,
                    0x7F => IInst::Li,
                    0x0F => IInst::Lui,
                    0x23 => IInst::Lw,
                    0x0D => IInst::Ori,
                    0x0A => IInst::Slti,
                    0x0B => IInst::Sltiu,
                    0x28 => IInst::Sb,
                    0x38 => IInst::Sc,
                    0x29 => IInst::Sh,
                    0x2B => IInst::Sw,
                    _    => unimplemented!(),
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
                    IInst::Addi => 0x08,
                    IInst::Addiu => 0x09,
                    IInst::Andi => 0x0C,
                    IInst::Beq => 0x04,
                    IInst::Bne => 0x05,
                    IInst::Lbu => 0x24,
                    IInst::Lhu => 0x25,
                    IInst::Ll => 0x30,
                    IInst::Li => 0x7F,
                    IInst::Lui => 0x0F,
                    IInst::Lw => 0x23,
                    IInst::Ori => 0x0D,
                    IInst::Slti => 0x0A,
                    IInst::Sltiu => 0x0B,
                    IInst::Sb => 0x28,
                    IInst::Sc => 0x38,
                    IInst::Sh => 0x29,
                    IInst::Sw => 0x2B,
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
    imm: u16,
}

impl RType {
    pub fn new<T,U,W,Q>(rs: T, rt: U, rd: Q, shamt: W, funct: RInst) -> RType where Reg: From<T> + From<U> + From<Q>, u8: From<W> {
        RType {rs: Reg::from(rs), rt: Reg::from(rt), rd: Reg::from(rd), shamt: u8::from(shamt), funct}
    }
    pub fn perform(&self, state: &mut State) {
        let rs = state.read_reg(self.rs);
        let rt = state.read_reg(self.rt);
        match self.funct {
           RInst::Add => state.write_reg(self.rd, i32::wrapping_add(rs as i32, rt as i32) as u32),
           RInst::Addu => state.write_reg(self.rd, u32::wrapping_add(rs, rt)),
           RInst::And => state.write_reg(self.rd, rs & rt),
           RInst::Jr => unimplemented!(),
           RInst::Nor => state.write_reg(self.rd, !(rs | rt)),
           RInst::Or => state.write_reg(self.rd, rs | rt),
           RInst::Slt => state.write_reg(self.rd, match (rs as i32) < (rt as i32) { true => 1u32, false => 0u32 }),
           RInst::Sltu => state.write_reg(self.rd, match rs < rt { true => 1u32, false => 0u32 }),
           RInst::Sll => state.write_reg(self.rd, rt << self.shamt),
           RInst::Srl => state.write_reg(self.rd, rt >> self.shamt),
           RInst::Sub => state.write_reg(self.rd, i32::wrapping_sub(rs as i32, rt as i32) as u32),
           RInst::Subu => state.write_reg(self.rd, u32::wrapping_sub(rs, rt)),
           RInst::Div => state.write_reg(self.rd, ((rs as i32) / (rt as i32)) as u32),
           RInst::Divu => state.write_reg(self.rd, rs / rt),
        }
    }
}

impl Into<u32> for RType {
    fn into(self) -> u32 {
        let mut x = 0u32;
        x |= u32::from(self.rs) << 21;
        x |= u32::from(self.rt) << 16;
        x |= u32::from(self.rd) << 11;
        x |= (self.shamt as u32) << 6;
        x |= u32::from(self.funct);
        x
    }
}

impl IType {
    pub fn new<T,U,Q>(opcode: IInst, rs: T, rt: U, imm: Q) -> IType where Reg: From<T> + From<U>, u16: From<Q> {
        IType {opcode, rs: Reg::from(rs), rt: Reg::from(rt), imm: u16::from(imm)}
    }
    pub fn perform(&self, state: &mut State) {
        let rs = state.read_reg(self.rs);
        let rt = state.read_reg(self.rt);
        let imm = self.imm as u32;
        match self.opcode {
           IInst::Addi => state.write_reg(self.rt, i32::wrapping_add(rs as i32, imm as i32) as u32),
           IInst::Addiu => state.write_reg(self.rt, u32::wrapping_add(rs, imm)),
           IInst::Andi => state.write_reg(self.rt, rs & imm),
           IInst::Beq => unimplemented!(),
           IInst::Bne => unimplemented!(),
           IInst::Lbu => state.write_reg(self.rt, state.read_mem(u32::wrapping_add(rs, imm)) & 0xFFu32),
           IInst::Lhu => state.write_reg(self.rt, state.read_mem(u32::wrapping_add(rs, imm)) & 0xFFFFu32),
           IInst::Ll | IInst::Lw => state.write_reg(self.rt, state.read_mem(u32::wrapping_add(rs, imm))),
           IInst::Li => state.write_reg(self.rt, imm),
           IInst::Lui => state.write_reg(self.rt, imm << 16),
           IInst::Ori => state.write_reg(self.rt, rs | imm),
           IInst::Slti => state.write_reg(self.rt, match (rs as i32) < (imm as i32) { true => 1u32, false => 0u32 }),
           IInst::Sltiu => state.write_reg(self.rt, match rs < imm { true => 1u32, false => 0u32 }),
           IInst::Sb => state.write_mem(u32::wrapping_add(rs, imm), rt & 0xFFu32),
           IInst::Sc => unimplemented!(),
           IInst::Sh => state.write_mem(u32::wrapping_add(rs, imm), rt & 0xFFFFu32),
           IInst::Sw => state.write_mem(u32::wrapping_add(rs, imm), rt),
        }
    }
}

impl Into<u32> for IType {
    fn into(self) -> u32 {
        let mut x = 0u32;
        x |= u32::from(self.opcode) << 26;
        x |= u32::from(self.rs) << 21;
        x |= u32::from(self.rt) << 16;
        x |= u32::from(self.imm);
        x
    }
}

pub fn main() {
    let mut state = State::new();
    let load1 = IType::new(IInst::Li, 0u8, Reg::t0, 10u8);
    let add = RType::new(Reg::t0, Reg::t0, Reg::t0, 0u8, RInst::Add);
    load1.perform(&mut state);
    add.perform(&mut state);
    println!("registers:\n{:?}", state);
}

