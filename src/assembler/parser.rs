use std::vec::Vec;
use std::ops::{Index, IndexMut};

#[derive(Clone, Debug)]
pub struct Parser {
    data_segment: Segment,
    text_segment: Segment,
}

#[derive(Clone, Debug)]
struct SegmentEntry {
    offset: u32,          // offset from segment start
    alignment: Alignment, // alignment of each entry
    data: Vec<[u8; 4]>,   // size of an entry is data.len() * Alignment
                          // Accessing a data element is based off the alignment
}

trait ToFromBytes {
    fn to_bytes(&self) -> [u8; 4];
    fn from_bytes(bytes: [u8; 4]) -> Self;
}

impl ToFromBytes for u8 {
    fn to_bytes(&self) -> [u8; 4] {
        let mut out = [0u8; 4];
        out.clone_from_slice(&self.to_le_bytes());
        out
    }
    fn from_bytes(bytes: [u8; 4]) -> Self {
        bytes[0]
    }
}

impl ToFromBytes for u16 {
    fn to_bytes(&self) -> [u8; 4] {
        let mut out = [0u8; 4];
        out.clone_from_slice(&self.to_le_bytes());
        out
    }
    fn from_bytes(bytes: [u8; 4]) -> Self {
        Self::from_le_bytes([bytes[0], bytes[1]])
    }
}

impl ToFromBytes for u32 {
    fn to_bytes(&self) -> [u8; 4] {
        let mut out = [0u8; 4];
        out.clone_from_slice(&self.to_le_bytes());
        out
    }
    fn from_bytes(bytes: [u8; 4]) -> Self {
        Self::from_le_bytes(bytes)
    }
}

impl SegmentEntry {
    fn new<T,U,V>(offset: T, alignment: U, data: &[V]) -> SegmentEntry where u32: From<T>, Alignment: From<U>, V: ToFromBytes {
        if data.len() == 0 {
            SegmentEntry {offset: offset.into(), alignment: alignment.into(), data: Vec::new()}
        } else {
            let mut v: Vec<[u8; 4]> = Vec::with_capacity(data.len());
            for d in data {
                v.push(d.to_bytes());
            }
            SegmentEntry {offset: offset.into(), alignment: alignment.into(), data: v}
        }
    }
    fn add_data<T>(&mut self, data: &T) where T: ToFromBytes {
        self.data.push(data.to_bytes());
    }
    fn get_data_checked(&self, idx: usize) -> Option<&[u8]> {
        if idx >= self.data.len() {
            None
        } else {
            Some(&self[idx])
        }
    }
    fn get_data_mut_checked(&mut self, idx: usize) -> Option<&mut [u8]> {
        if idx >= self.data.len() {
            None
        } else {
            Some(&mut self[idx])
        }
    }
}

impl Index<usize> for SegmentEntry {
    type Output = [u8];
    fn index(&self, idx: usize) -> &Self::Output {
        match self.alignment {
            Alignment::Byte     => &self.data[idx][..1],
            Alignment::HalfWord => &self.data[idx][..2],
            Alignment::Word     => &self.data[idx][..4],
        }
    }
}

impl IndexMut<usize> for SegmentEntry {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match self.alignment {
            Alignment::Byte     => &mut self.data[idx][..1],
            Alignment::HalfWord => &mut self.data[idx][..2],
            Alignment::Word     => &mut self.data[idx][..4],
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Alignment {
    Byte,
    HalfWord,
    Word,
}

macro_rules! alignment_map {
    ($type_name: ty) => (
        impl From<$type_name> for Alignment {
            fn from(n: $type_name) -> Alignment {
                match n {
                    1 => Alignment::Byte,
                    2 => Alignment::HalfWord,
                    4 => Alignment::Word,
                    _ => panic!("Invalid alignemnt: {}", n)
                }
            }
        }
    );
}

macro_rules! alignment_inv_map {
    ($type_name: ty) => (
        impl From<Alignment> for $type_name {
            fn from(a: Alignment) -> Self {
                match a {
                    Alignment::Byte => 1,
                    Alignment::HalfWord => 2,
                    Alignment::Word => 4,
                }
            }
        }
    );
}

alignment_map!(u8);
alignment_map!(u16);
alignment_map!(u32);
alignment_map!(u64);
alignment_map!(u128);
alignment_map!(usize);
alignment_inv_map!(u8);
alignment_inv_map!(u16);
alignment_inv_map!(u32);
alignment_inv_map!(u64);
alignment_inv_map!(u128);
alignment_inv_map!(usize);

#[derive(Clone, Debug)]
struct Segment {
    entries: Vec<SegmentEntry>
}
impl Segment {
    fn get_size(&self) -> u32 {
        let mut size: u32 = 0;
        self.entries.iter().for_each(|e| size += (e.data.len() * usize::from(e.alignment)) as u32);
        size
    }
    fn get_entries(&self) -> &[SegmentEntry] {
        &self.entries[..]
    }
}

