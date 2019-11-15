use std::{
    cmp::{Eq, PartialEq},
    fmt::Debug,
    mem,
    num::NonZeroUsize,
};

pub trait Compressable: Clone + Debug + Eq + PartialEq {}

#[derive(Clone, Debug, PartialEq)]
enum Entry<T>
where
    T: Compressable,
{
    Data(T),
    Ptr(Compressed<T>),
}

impl<T> Eq for Entry<T> where T: Compressable {}

#[derive(Clone, Debug, PartialEq)]
struct Compressed<T>
where
    T: Compressable,
{
    len: NonZeroUsize,
    data: T,
}

impl<T> Eq for Compressed<T> where T: Compressable {}

impl<T> Compressed<T>
where
    T: Compressable,
{
    pub fn new(data: T, len: NonZeroUsize) -> Compressed<T> {
        Compressed { data, len }
    }

    pub fn compress(data: &[T]) -> Vec<Entry<T>> {
        let size_T = mem::size_of::<T>();
        let size_C = mem::size_of::<Compressed<T>>();
        let mut counter: usize = 0;
        let mut current_item: Option<&T> = None;
        let mut v = Vec::new();
        for d in data {
            match current_item {
                None => {
                    current_item = Some(&d);
                    counter = 1;
                    continue;
                }
                Some(x) => {
                    if x == d {
                        counter += 1;
                        continue;
                    }
                }
            };
            if size_T * counter > size_C {
                unsafe {
                    v.push(Entry::Ptr(Compressed::new(
                        d.clone(),
                        NonZeroUsize::new_unchecked(counter),
                    )));
                }
                counter = 0;
                current_item = None;
            } else {
                while counter > 0 {
                    v.push(Entry::Data(d.clone()));
                    counter -= 1;
                }
                current_item = None;
            }
        }
        v
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

#[derive(Clone, Debug, PartialEq)]
pub struct CompressedVec<T>
where
    T: Compressable,
{
    data: Vec<Entry<T>>,
}

impl<T> CompressedVec<T>
where
    T: Compressable,
{
    pub fn new(data: &[T]) -> Self {
        CompressedVec {
            data: Compressed::compress(data),
        }
    }
}

impl<T> Eq for CompressedVec<T> where T: Compressable {}
