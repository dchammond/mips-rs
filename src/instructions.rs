pub mod itype;
pub mod jtype;
pub mod rtype;
//pub mod label;

#[derive(Clone, Debug)]
pub enum Inst {
    I(itype::IType),
    R(rtype::RType),
    J(jtype::JType),
}

impl From<itype::IType> for Inst {
    fn from(i: itype::IType) -> Self {
        Inst::I(i)
    }
}

impl From<rtype::RType> for Inst {
    fn from(r: rtype::RType) -> Self {
        Inst::R(r)
    }
}

impl From<jtype::JType> for Inst {
    fn from(j: jtype::JType) -> Self {
        Inst::J(j)
    }
}
