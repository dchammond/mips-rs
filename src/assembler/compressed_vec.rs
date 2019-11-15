use std::{fmt::Debug, num::NonZeroUsize};

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
}

#[derive(Clone, Debug)]
pub struct CompressedVec<T>
where
    T: Clone + Debug,
{
    data: Vec<Entry<T>>,
}

impl<T> CompressedVec<T> where T: Clone + Debug {}
