#[derive(Copy, Clone, Debug)]
pub enum Imm {
    Raw(u32),
    Label(u32),
    Address(u32),
}

macro_rules! imm_map {
    ($type_name: ty) => {
        impl From<$type_name> for Imm {
            fn from(n: $type_name) -> Imm {
                Imm::Raw(n as u32)
            }
        }
    };
}

imm_map!(u8);
imm_map!(u16);
imm_map!(u32);
imm_map!(u64);
imm_map!(u128);

macro_rules! imm_inv_map {
    ($type_name: ty) => {
        impl From<Imm> for $type_name {
            fn from(i: Imm) -> $type_name {
                match i {
                    Imm::Raw(r) => r as $type_name,
                    Imm::Label(l) => l as $type_name,
                    Imm::Address(a) => a as $type_name,
                }
            }
        }
    };
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
