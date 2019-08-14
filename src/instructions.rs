//pub mod itype;
pub mod rtype;
//pub mod jtype;
//pub mod label;

#[derive(Clone, Debug)]
pub enum Inst {
    R(rtype::RType)
}

impl From<rtype::RType> for Inst {
    fn from(r: rtype::RType) -> Self {
        Inst::R(r)
    }
}

