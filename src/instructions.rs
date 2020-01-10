pub mod itype;
pub mod jtype;
pub mod rtype;

#[derive(Clone, Debug)]
pub enum Inst {
    IImm(itype::ITypeImm),
    ILabel(itype::ITypeLabel),
    R(rtype::RType),
    J(jtype::JType),
}

impl From<itype::ITypeImm> for Inst {
    fn from(i: itype::ITypeImm) -> Self {
        Inst::IImm(i)
    }
}

impl From<itype::ITypeLabel> for Inst {
    fn from(i: itype::ITypeLabel) -> Self {
        Inst::ILabel(i)
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
