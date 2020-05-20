use crate::machine::memory;

#[repr(C)]
#[repr(packed)]
struct E_Ident {
    magic: [u8; 4],
    class: u8,
    endian: u8,
    version: u8,
    osabi: u8,
    padding: [u8; 8],
}

const E_IDENT_MAGIC:         [u8; 4] = [0x7F, b'E', b'L', b'F'];
const E_IDENT_CLASS_32:      u8      = 1;
const E_IDENT_CLASS_64:      u8      = 2;
const E_IDENT_ENDIAN_LE:     u8      = 1;
const E_IDENT_ENDIAN_BE:     u8      = 2;
const E_IDENT_VERSION:       u8      = 1;
const E_IDENT_OSABI_SYSTEMV: u8      = 0;

#[repr(C)]
#[repr(packed)]
struct E_Type {
    type_: u16,
}

const E_TYPE_NONE:   u16 = 0;
const E_TYPE_REL:    u16 = 1;
const E_TYPE_EXEC:   u16 = 2;
const E_TYPE_DYN:    u16 = 3;
const E_TYPE_CORE:   u16 = 4;
const E_TYPE_LOPROC: u16 = 0xFF00;
const E_TYPE_HIPROC: u16 = 0xFFFF;

#[repr(C)]
#[repr(packed)]
struct E_Machine {
    machine: u16,
}

const E_MACHINE_MIPS32: u16 = 8;
const E_MACHINE_MIPS64: u16 = 10;

#[repr(C)]
#[repr(packed)]
struct E_Version {
    version: u32,
}

const E_VERSION: u32 = 1;

#[repr(C)]
#[repr(packed)]
struct E_Entry {
    entry: u32,
}

// Don't want to start executing at null for sake of sanity
// Eventually this will become a virtual address
const E_ENTRY: u32 = memory::BOTTOM_RESERVED_START + 4;

#[repr(C)]
#[repr(packed)]
struct E_Phoff {
    phoff: u32,
}

const E_PHOFF: u32 = 0x34;

#[repr(C)]
#[repr(packed)]
struct E_Shoff {
    shoff: u32,
}

// TODO: Calculated from size of text and data segments in object file

#[repr(C)]
#[repr(packed)]
struct E_Flags {
    flags: u32,
}

const E_FLAGS_MIPS_NOREORDER: u32 = 0x0000_0001;
const E_FLAGS_MIPS_PIC:       u32 = 0x0000_0002;
const E_FLAGS_MIPS_CPIC:      u32 = 0x0000_0004;
const E_FLAGS_MIPS_ARCH:      u32 = 0xF000_0000;

#[repr(C)]
#[repr(packed)]
struct E_Ehsize {
    ehsize: u16,
}

const E_EHSIZE: u16 = std::mem::size_of::<Elf_Header>() as u16;

#[repr(C)]
#[repr(packed)]
struct E_Phentsize {
    phentsize: u16,
}

#[repr(C)]
#[repr(packed)]
struct E_Phnum {
    phnum: u16,
}

#[repr(C)]
#[repr(packed)]
struct E_Shentsize {
    shentsize: u16,
}

#[repr(C)]
#[repr(packed)]
struct E_Shnum {
    shnum: u16,
}

#[repr(C)]
#[repr(packed)]
struct E_Shstrndx {
    shstrndx: u16,
}
#[repr(C)]
#[repr(packed)]
pub struct Elf_Header {
    e_ident: E_Ident,
    e_type: E_Type,
    e_machine: E_Machine,
    e_version: E_Version,
    e_entry: E_Entry,
    e_phoff: E_Phoff,
    e_shoff: E_Shoff,
    e_flags: E_Flags,
    e_ehsize: E_Ehsize,
    e_phentsize: E_Phentsize,
    e_phnum: E_Phnum,
    e_shentsize: E_Shentsize,
    e_shnum: E_Shnum,
    e_shstrndx: E_Shstrndx,
}
