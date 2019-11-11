use lazy_static::lazy_static;
use regex::Regex;

use std::fmt;

use crate::instructions::itype::IType;
use crate::instructions::jtype::{JInst, JType};
use crate::instructions::label::Label;
use crate::instructions::rtype::RType;

#[derive(Clone)]
pub struct State {
    pc: u32,
    registers: [u32; 32],
    memory: [u32; std::u16::MAX as usize],
    labels: Vec<Label>,
}

#[derive(Clone)]
pub enum InstType {
    R(RType),
    I(IType),
    J(JType),
}

impl State {
    pub fn new() -> Self {
        State {
            pc: 0,
            registers: [0; 32],
            memory: [0; std::u16::MAX as usize],
            labels: Vec::new(),
        }
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
                }
                InstType::I(i) => {
                    i.perform(self);
                }
                InstType::J(j) => {
                    j.perform(self);
                }
            }
            if self.pc == 0 {
                break;
            }
        }
    }
    pub fn parse_instruction<T>(inst: T) -> InstType
    where
        u32: From<T>,
    {
        let inst: u32 = inst.into();
        let opcode: u32 = inst;
        let opcode: u32 = opcode >> 26;
        if opcode == 0 {
            InstType::R(RType::from(inst))
        } else if opcode == JInst::j.into() || opcode == JInst::jal.into() {
            InstType::J(JType::from(inst))
        } else {
            InstType::I(IType::from(inst))
        }
    }
    pub fn load_compiled_instructions<T>(&mut self, instructions: &[u32], start: Option<T>)
    where
        u32: From<T>,
    {
        let mut start: u32 = match start {
            Some(s) => s.into(),
            None => 0x4,
        };
        for inst in instructions {
            self.memory[start as usize] = *inst;
            start += 4;
        }
    }
    pub fn load_parsed_instructions<T>(&mut self, instructions: &[InstType], start: Option<T>)
    where
        u32: From<T>,
    {
        let mut start: u32 = match start {
            Some(s) => s.into(),
            None => 0x4,
        };
        for inst in instructions {
            self.memory[start as usize] = match *inst {
                InstType::R(r) => r.into(),
                InstType::I(i) => i.into(),
                InstType::J(j) => j.into(),
            };
            start += 4;
        }
    }
    pub fn load_text_instructions<T>(&mut self, instructions: &[&str], start: Option<T>)
    where
        u32: From<T>,
    {
        lazy_static! {
            static ref LABEL_RE: Regex = Regex::new(r"^\s*(?P<label>\w+):\s*$").unwrap();
        }
        let mut start: u32 = match start {
            Some(s) => s.into(),
            None => 0,
        };
        let mut labels: Vec<u32> = Vec::new();
        {
            let mut count = start;
            for line in instructions {
                let trim = line.trim();
                if trim == "" {
                    continue;
                }
                for caps in LABEL_RE.captures_iter(trim) {
                    self.add_label::<u32, &str>(Some(count), &caps["label"]);
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
    pub fn read_pc(&self) -> u32 {
        self.pc
    }
    pub fn read_reg<T>(&self, r: T) -> u32
    where
        u8: From<T>,
    {
        self.registers[u8::from(r) as usize]
    }
    pub fn dump_reg(&self) -> [u32; 33] {
        let mut r: [u32; 33] = [0; 33];
        r.clone_from_slice(&self.registers[..]);
        r[32] = self.pc;
        r
    }
    pub fn write_reg<T, U>(&mut self, r: T, val: U)
    where
        u8: From<T>,
        u32: From<U>,
    {
        let reg = u8::from(r);
        match reg {
            0 => (),
            _ => self.registers[reg as usize] = u32::from(val),
        };
    }
    pub fn jump<T>(&mut self, dest: T)
    where
        u32: From<T>,
    {
        self.pc = u32::from(dest);
    }
    pub fn write_mem<T, U>(&mut self, addr: T, val: U)
    where
        u32: From<T> + From<U>,
    {
        self.memory[u32::from(addr) as usize] = u32::from(val);
    }
    pub fn read_mem<T>(&self, addr: T) -> u32
    where
        u32: From<T>,
    {
        self.memory[u32::from(addr) as usize]
    }
    pub fn find_label_by_addr<T>(&self, addr: T) -> Option<String>
    where
        u32: From<T>,
    {
        let x = u32::from(addr);
        for p in &self.labels {
            match p.addr {
                Some(a) => {
                    if a == x {
                        return Some(p.label.clone());
                    }
                }
                None => (),
            }
        }
        None
    }
    pub fn find_label_by_name<T>(&self, name: T) -> Option<u32>
    where
        String: From<T>,
    {
        let x = String::from(name);
        for p in &self.labels {
            if p.label == x {
                return p.addr;
            }
        }
        None
    }
    pub fn add_label<T, U>(&mut self, addr: Option<T>, label: U)
    where
        u32: From<T>,
        String: From<U>,
        U: Clone,
    {
        let addr: Option<u32> = match addr {
            Some(a) => Some(a.into()),
            None => None,
        };
        let label = String::from(label);
        for p in &mut self.labels {
            if p.label == label {
                match p.addr {
                    Some(_) => {
                        return;
                    }
                    None => {
                        p.addr = addr;
                        return;
                    }
                }
            }
        }
        self.labels.push(Label::new::<u32, String>(addr, label))
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "$pc: 0x{:08X} == {}\n", self.pc, self.pc)?;
        write!(
            f,
            "$0 : 0x{:08X} == {}\n",
            self.registers[0], self.registers[0]
        )?;
        write!(
            f,
            "$at: 0x{:08X} == {}\n",
            self.registers[1], self.registers[1]
        )?;
        write!(
            f,
            "$v0: 0x{:08X} == {}\n",
            self.registers[2], self.registers[2]
        )?;
        write!(
            f,
            "$v1: 0x{:08X} == {}\n",
            self.registers[3], self.registers[3]
        )?;
        write!(
            f,
            "$a0: 0x{:08X} == {}\n",
            self.registers[4], self.registers[4]
        )?;
        write!(
            f,
            "$a1: 0x{:08X} == {}\n",
            self.registers[5], self.registers[5]
        )?;
        write!(
            f,
            "$a2: 0x{:08X} == {}\n",
            self.registers[6], self.registers[6]
        )?;
        write!(
            f,
            "$a3: 0x{:08X} == {}\n",
            self.registers[7], self.registers[7]
        )?;
        write!(
            f,
            "$t0: 0x{:08X} == {}\n",
            self.registers[8], self.registers[8]
        )?;
        write!(
            f,
            "$t1: 0x{:08X} == {}\n",
            self.registers[9], self.registers[9]
        )?;
        write!(
            f,
            "$t2: 0x{:08X} == {}\n",
            self.registers[10], self.registers[10]
        )?;
        write!(
            f,
            "$t3: 0x{:08X} == {}\n",
            self.registers[11], self.registers[11]
        )?;
        write!(
            f,
            "$t4: 0x{:08X} == {}\n",
            self.registers[12], self.registers[12]
        )?;
        write!(
            f,
            "$t5: 0x{:08X} == {}\n",
            self.registers[13], self.registers[13]
        )?;
        write!(
            f,
            "$t6: 0x{:08X} == {}\n",
            self.registers[14], self.registers[14]
        )?;
        write!(
            f,
            "$t7: 0x{:08X} == {}\n",
            self.registers[15], self.registers[15]
        )?;
        write!(
            f,
            "$s0; 0x{:08X} == {}\n",
            self.registers[16], self.registers[16]
        )?;
        write!(
            f,
            "$s1: 0x{:08X} == {}\n",
            self.registers[17], self.registers[17]
        )?;
        write!(
            f,
            "$s2: 0x{:08X} == {}\n",
            self.registers[18], self.registers[18]
        )?;
        write!(
            f,
            "$s3: 0x{:08X} == {}\n",
            self.registers[19], self.registers[19]
        )?;
        write!(
            f,
            "$s4: 0x{:08X} == {}\n",
            self.registers[20], self.registers[20]
        )?;
        write!(
            f,
            "$s5: 0x{:08X} == {}\n",
            self.registers[21], self.registers[21]
        )?;
        write!(
            f,
            "$s6: 0x{:08X} == {}\n",
            self.registers[22], self.registers[22]
        )?;
        write!(
            f,
            "$s7: 0x{:08X} == {}\n",
            self.registers[23], self.registers[23]
        )?;
        write!(
            f,
            "$t8: 0x{:08X} == {}\n",
            self.registers[24], self.registers[24]
        )?;
        write!(
            f,
            "$t9: 0x{:08X} == {}\n",
            self.registers[25], self.registers[25]
        )?;
        write!(
            f,
            "$sp: 0x{:08X} == {}\n",
            self.registers[29], self.registers[29]
        )?;
        write!(
            f,
            "$fp: 0x{:08X} == {}\n",
            self.registers[30], self.registers[30]
        )?;
        write!(
            f,
            "$ra: 0x{:08X} == {}",
            self.registers[31], self.registers[31]
        )
    }
}
