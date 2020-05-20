#[repr(C)]
#[repr(packed)]
struct P_Type {
    type_: u32,
}

const P_TYPE_NULL:    u32 = 0;
const P_TYPE_LOAD:    u32 = 1;
const P_TYPE_DYNAMIC: u32 = 2;
const P_TYPE_INTERP:  u32 = 3;
const P_TYPE_NOTE:    u32 = 4;
const P_TYPE_SHLIB:   u32 = 5;
const P_TYPE_PHDR:    u32 = 6;
const P_TYPE_TLS:     u32 = 7;
const P_TYPE_LOPROC:  u32 = 0x7000_0000;
const P_TYPE_HIPROC:  u32 = 0x7FFF_FFFF;

#[repr(C)]
#[repr(packed)]
struct P_Offset {
    offset: u32,
}
// Byte offset into file where the segment resides

#[repr(C)]
#[repr(packed)]
struct P_Vaddr {
    vaddr: u32,
}
// Address of this segment in memory

#[repr(C)]
#[repr(packed)]
struct P_Paddr {
    paddr: u32,
}
// Pysical address of this segment in memory
// if applicable

#[repr(C)]
#[repr(packed)]
struct P_Filesz {
    filesz: u32,
}
// Size of this segment in the file

#[repr(C)]
#[repr(packed)]
struct P_Memsz {
    memsz: u32,
}
// Size of this segment in memory

#[repr(C)]
#[repr(packed)]
struct P_Flags {
    flags: u32,
}

const P_FLAGS_NONE:  u32 = 0x0000_0000;
const P_FLAGS_EXEC:  u32 = 0x0000_0001;
const P_FLAGS_WRITE: u32 = 0x0000_0002;
const P_FLAGS_READ:  u32 = 0x0000_0004;

#[repr(C)]
#[repr(packed)]
struct P_Align {
    align: u32,
}
// p_vaddr % p_align == p_offset

const P_ALIGN_NONE0: u32 = 0;
const P_ALIGN_NONE1: u32 = 1;

#[repr(C)]
#[repr(packed)]
pub(in crate::elf) struct Program_Header {
    p_type: P_Type,
    p_offset: P_Offset,
    p_vaddr: P_Vaddr,
    p_paddr: P_Paddr,
    p_filesz: P_Filesz,
    p_memsz: P_Memsz,
    p_flags: P_Flags,
    p_align: P_Align,
}
