#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum Reg {
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

