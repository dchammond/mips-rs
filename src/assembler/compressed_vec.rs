use std::{fmt::Debug, num::NonZeroUsize, mem};

#[derive(Clone, Debug)]
enum Entry<T>
where
    T: Clone + Debug,
{
    Data(T),
    Ptr(Compressed<T>),
}

#[derive(Clone, Debug)]
struct Compressed<T>
where
    T: Clone + Debug,
{
    len: NonZeroUsize,
    data: T,
}

impl<T> Compressed<T> where T: Clone + Debug {
    pub fn new(data: T, len: NonZeroUsize) -> Compressed<T> {
        Compressed { data, len }
    }

    pub fn get(&self) -> T {
        self.data.clone()
    }

    pub fn get_ref(&self) -> &T {
        &self.data
    }

    pub fn size_bytes(&self) -> usize {
        self.len.get() as usize * mem::size_of::<T>()
    }
}

#[derive(Clone, Debug)]
pub struct CompressedVec<T>
where
    T: Clone + Debug,
{
    data: Vec<Entry<T>>,
}

impl<T> CompressedVec<T> where T: Clone + Debug {}
