#[repr(C)]
#[repr(packed)]
struct E_Ident {
    magic: [u8; 4],
    class: u8,
    endian: u8,
    version: u8,
    osabi: u8,
    abiversion: u8,
    padding: [u8; 7],
}

#[repr(C)]
#[repr(packed)]
struct E_Type {
    type_: u16,
}

#[repr(C)]
#[repr(packed)]
struct E_Machine {
    machine: u16,
}

#[repr(C)]
#[repr(packed)]
struct E_Version {
    version: u32,
}

#[repr(C)]
#[repr(packed)]
struct E_Entry {
    entry: u32,
}

#[repr(C)]
#[repr(packed)]
struct E_Phoff {
    phoff: u32,
}

#[repr(C)]
#[repr(packed)]
struct E_Shoff {
    shoff: u32,
}

#[repr(C)]
#[repr(packed)]
struct E_Flags {
    flags: u32,
}

#[repr(C)]
#[repr(packed)]
struct E_Ehsize {
    ehsize: u16,
}

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
pub struct Header {
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
