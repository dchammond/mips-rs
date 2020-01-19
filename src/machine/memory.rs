use std::{fmt::Debug, num::NonZeroU32};

// grows down
pub const TOP_RESERVED_SIZE: u32 = 0x0000_FFEF;
pub const TOP_RESERVED_START: u32 = 0xFFFF_FFFF;
pub const TOP_RESERVED_END: u32 = TOP_RESERVED_START - TOP_RESERVED_SIZE;

pub const MMIO_SIZE: u32 = 0x0000_0010;
pub const MMIO_START: u32 = TOP_RESERVED_END;
pub const MMIO_END: u32 = MMIO_START - MMIO_SIZE;

pub const KERNEL_DATA_SIZE: u32 = 0x6FFF_0000;
pub const KERNEL_DATA_START: u32 = MMIO_END;
pub const KERNEL_DATA_END: u32 = KERNEL_DATA_START - KERNEL_DATA_SIZE;

pub const KERNEL_TEXT_SIZE: u32 = 0x1000_0000;
pub const KERNEL_TEXT_START: u32 = KERNEL_DATA_END;
pub const KERNEL_TEXT_END: u32 = KERNEL_TEXT_START - KERNEL_TEXT_SIZE;

pub const STACK_START: u32 = KERNEL_TEXT_END;
// grows up
pub const STATIC_DATA_START: u32 = 0x1000_0000;

pub const TEXT_SIZE: u32 = 0x0600_0000;
pub const TEXT_END: u32 = STATIC_DATA_START;
pub const TEXT_START: u32 = TEXT_END - TEXT_SIZE;

pub const BOTTOM_RESERVED_SIZE: u32 = 0x0400_0000;
pub const BOTTOM_RESERVED_END: u32 = TEXT_START;
pub const BOTTOM_RESERVED_START: u32 = BOTTOM_RESERVED_END - BOTTOM_RESERVED_SIZE;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MemRangeStatus {
    Free,
    Allocated,
}

impl Eq for MemRangeStatus {}

#[derive(Clone, Debug)]
pub struct MemRange<'a, T> {
    lower: u32,
    upper: u32, // inclusive
    status: MemRangeStatus,
    data: Option<&'a T>,
}

impl<'a, T> MemRange<'a, T>
where
    T: Clone + Debug,
{
    pub fn new(lower: u32, upper: u32, status: MemRangeStatus, data: Option<&T>) -> MemRange<T> {
        if lower > upper {
            panic!("lower > upper");
        }
        MemRange {
            lower,
            upper,
            status,
            data,
        }
    }
    pub fn get_range(&self) -> (u32, u32) {
        (self.lower, self.upper)
    }
    pub fn get_status(&self) -> MemRangeStatus {
        self.status
    }
    pub fn get_data(&self) -> Option<&'a T> {
        self.data
    }
    pub fn set_range(self, lower: u32, upper: u32) -> MemRange<'a, T> {
        MemRange::new(lower, upper, self.status, self.data)
    }
    pub fn set_status(&mut self, status: MemRangeStatus) {
        self.status = status;
    }
    pub fn set_data(&mut self, data: Option<&'a T>) {
        self.data = data;
    }
    pub fn size_bytes(&self) -> u32 {
        self.upper - self.lower + 1
    }
    pub fn shrink<'b>(mut self, bytes: NonZeroU32) -> (MemRange<'a, T>, MemRange<'b, T>) {
        self.upper -= bytes.get();
        (
            self,
            MemRange::new(
                self.upper + 1,
                self.upper + 1 + bytes.get(),
                MemRangeStatus::Free,
                None,
            ),
        )
    }
    pub fn grow<'b>(
        mut self,
        next: Option<&'b [MemRange<'b, T>]>,
        bytes: NonZeroU32,
    ) -> (MemRange<'a, T>, Option<Vec<MemRange<'b, T>>>) {
        self.upper += bytes.get();
        let mut r = None;
        if let Some(ranges) = next {
            let mut tmp = ranges
                .into_iter()
                .cloned()
                .skip_while(|mem| {
                    if MemRangeStatus::Free != mem.status {
                        panic!("Tried to grow into non-free block: {:?} -> {:?}", self, mem);
                    }
                    mem.upper < self.upper
                })
                .collect::<Vec<MemRange<'b, T>>>();
            if tmp.len() > 0 {
                let next = tmp[0];
                tmp[0] = MemRange::new(self.upper + 1, next.upper, MemRangeStatus::Free, None);
            }
            r = Some(tmp);
        }
        (self, r)
    }
    /**
     * "Middle" will always be returned as the second element of the tuple
     * The first element exists if there is a MemRange lower than middle
     * The third element exists if there is a MemRange higher than middle
     */
    pub fn insert<'b, 'c, 'd>(
        self,
        middle: MemRange<'c, T>,
    ) -> (
        Option<MemRange<'b, T>>,
        MemRange<'c, T>,
        Option<MemRange<'d, T>>,
    ) {
        if MemRangeStatus::Free != self.status {
            panic!(
                "Tried to insert into non-free block: {:?} <-> {:?}",
                self, middle
            );
        }
        if middle.lower < self.lower || self.upper < middle.upper {
            panic!("Tried to insert out of bounds: {:?} <- {:?}", self, middle);
        }
        // TODO: clenaup and use match like in first_fit
        if middle.lower == self.lower && middle.upper == self.upper {
            (None, middle, None)
        } else if middle.lower == self.lower {
            (
                None,
                middle,
                Some(MemRange::new(
                    middle.upper + 1,
                    self.upper,
                    MemRangeStatus::Free,
                    None,
                )),
            )
        } else if middle.upper == self.upper {
            (
                Some(MemRange::new(
                    self.lower,
                    middle.lower - 1,
                    MemRangeStatus::Free,
                    None,
                )),
                middle,
                None,
            )
        } else {
            (
                Some(MemRange::new(
                    self.lower,
                    middle.lower - 1,
                    MemRangeStatus::Free,
                    None,
                )),
                middle,
                Some(MemRange::new(
                    middle.upper + 1,
                    self.upper,
                    MemRangeStatus::Free,
                    None,
                )),
            )
        }
    }
    pub fn merge<'b>(mut self, other: MemRange<'b, T>) -> MemRange<'a, T> {
        if self.upper + 1 != other.lower {
            panic!("Is there a hole? {:?} <-> {:?}", self, other);
        }
        if MemRangeStatus::Free != other.status {
            panic!(
                "Cannot merge with non-free memory block: {:?} <-> {:?}",
                self, other
            );
        }
        self.upper = other.upper;
        self
    }
}

impl<'a, T> Copy for MemRange<'a, T> where T: Clone {}

#[derive(Clone, Debug)]
pub struct MemPosition<'a, T> {
    lower: Option<u32>,
    upper: Option<u32>, // inclusive
    size: u32,
    data: Option<&'a T>,
}

impl<'a, T> Copy for MemPosition<'a, T> where T: Clone {}

impl<'a, T> MemPosition<'a, T>
where
    T: Clone + Debug,
{
    pub fn new(
        lower: Option<u32>,
        upper: Option<u32>,
        size: u32,
        data: Option<&T>,
    ) -> MemPosition<T> {
        match (lower, upper) {
            (Some(l), Some(u)) => {
                if l > u {
                    panic!("lower > upper");
                }
                if u - l + 1 != size {
                    panic!("size does not match address range");
                }
            },
            _ => {},
        }
        MemPosition {
            lower,
            upper,
            size,
            data,
        }
    }
    pub fn get_range(&self) -> (Option<u32>, Option<u32>) {
        (self.lower, self.upper)
    }
    pub fn get_data(&self) -> Option<&'a T> {
        self.data
    }
    pub fn set_range(self, lower: Option<u32>, upper: Option<u32>, size: u32) -> MemPosition<'a, T> {
        MemPosition::new(lower, upper, size, self.data)
    }
    pub fn set_data(&mut self, data: Option<&'a T>) {
        self.data = data;
    }
    pub fn size_bytes(&self) -> u32 {
        self.size
    }
}

pub struct Allocator<'a, T> {
    start: u32,
    end: u32, // inclusive
    data: Vec<MemRange<'a, T>>,
}

impl<'a, T> Allocator<'a, T> where T: Clone + Debug {
    fn first_fit_insert<'b>(position: MemPosition<'b, T>, memory: &mut Vec<MemRange<'b, T>>) {
        let l = position.lower.unwrap();
        let u = position.upper.unwrap();
        if let Some((i, m)) = memory
            .iter()
            .enumerate()
            .find(|(_, mem)| mem.lower <= l && u <= mem.upper && MemRangeStatus::Free == mem.status)
        {
            let result = m.insert(MemRange::new(
                l,
                u,
                MemRangeStatus::Allocated,
                position.data,
            ));
            memory.remove(i);
            if let Some(x) = result.2 {
                memory.insert(i, x);
            }
            memory.insert(i, result.1);
            if let Some(x) = result.0 {
                memory.insert(i, x);
            }
        } else {
            panic!(
                "Could not find placement for: {:?} in {:?}",
                position, memory
            );
        }
    }
    fn first_fit_insert_arbitray<'b>(
        position: MemPosition<'b, T>,
        memory: &mut Vec<MemRange<'b, T>>,
    ) {
        if let Some((i, m)) = memory.iter().enumerate().find(|(_, mem)| {
            MemRangeStatus::Free == mem.status && position.size <= mem.size_bytes()
        }) {
            let result = m.insert(MemRange::new(
                m.lower,
                m.lower + position.size - 1,
                MemRangeStatus::Allocated,
                position.data,
            ));
            memory.remove(i);
            if let Some(x) = result.2 {
                memory.insert(i, x);
            }
            memory.insert(i, result.1);
            if let Some(x) = result.0 {
                memory.insert(i, x);
            }
        } else {
            panic!(
                "Could not find placement for: {} bytes in {:?}",
                position.size, memory
            );
        }
    }
    pub fn first_fit<'b>(
        positions: &'b [MemPosition<'b, T>],
        min: u32,
        max: u32,
    ) -> Vec<MemRange<'b, T>> {
        let mut memory = vec![MemRange::new(min, max, MemRangeStatus::Free, None)];
        let mut arbitraries: Vec<MemPosition<'b, T>> = Vec::new();
        positions.into_iter().for_each(|pos| match pos {
            MemPosition {
                lower: Some(_l),
                upper: Some(_u),
                ..
            } => {
                Allocator::first_fit_insert(*pos, &mut memory);
            }
            MemPosition {
                lower: Some(l),
                upper: None,
                size,
                data,
            } => {
                let u = l + size - 1;
                let pos = MemPosition::new(Some(*l), Some(u), *size, *data);
                Allocator::first_fit_insert(pos, &mut memory);
            }
            MemPosition {
                lower: None,
                upper: Some(u),
                size,
                data,
            } => {
                let l = u + 1 - size;
                let pos = MemPosition::new(Some(l), Some(*u), *size, *data);
                Allocator::first_fit_insert(pos, &mut memory);
            }
            _ => {
                arbitraries.push(*pos);
            }
        });
        arbitraries.into_iter().for_each(|pos| {
            Allocator::first_fit_insert_arbitray(pos, &mut memory);
        });
        memory
    }
}
