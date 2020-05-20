#[repr(C)]
#[repr(packed)]
struct S_Name {
    name: u32,
}
// Index of section name in section header string table

#[repr(C)]
#[repr(packed)]
struct S_Type {
    type_: u32,
}

const S_TYPE_NULL:     u32 = 0;
const S_TYPE_PROGBITS: u32 = 1;
const S_TYPE_SYMTAB:   u32 = 2;
const S_TYPE_STRTAB:   u32 = 3;
const S_TYPE_RELA:     u32 = 4;
const S_TYPE_HASH:     u32 = 5;
const S_TYPE_DYNAMIC:  u32 = 6;
const S_TYPE_NOTE:     u32 = 7;
const S_TYPE_NOBITS:   u32 = 8;
const S_TYPE_REL:      u32 = 9;
const S_TYPE_SHLIB:    u32 = 10;
const S_TYPE_DYNSYM:   u32 = 11;
const S_TYPE_LOPROC:   u32 = 0x7000_0000;
const S_TYPE_HIPROC:   u32 = 0x7FFF_FFFF;
const S_TYPE_LOUSER:   u32 = 0x8000_0000;
const S_TYPE_HIUSER:   u32 = 0x8FFF_FFFF;

#[repr(C)]
#[repr(packed)]
struct S_Flags {
    flags: u32,
}

const S_FLAGS_NONE:       u32 = 0x0000_0000;
const S_FLAGS_WRITE:      u32 = 0x0000_0001;
const S_FLAGS_ALLOC:      u32 = 0x0000_0002;
const S_FLAGS_EXECINSTR:  u32 = 0x0000_0004;
const S_FLAGS_MIPS_GPREL: u32 = 0x1000_0000;

#[repr(C)]
#[repr(packed)]
struct S_Addr {
    addr: u32,
}
// Address of section in memory else 0

#[repr(C)]
#[repr(packed)]
struct S_Offset {
    offset: u32,
}


#[repr(C)]
#[repr(packed)]
pub(in crate::elf) struct Section_Header {
}
