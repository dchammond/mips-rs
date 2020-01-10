use std::num::NonZeroU32;

#[derive(Clone, Debug)]
pub struct Address {
    pub numeric: Option<NonZeroU32>,
    pub label: Option<Vec<String>>,
}

impl Address {
    pub fn new(numeric: Option<NonZeroU32>, label: Option<Vec<String>>) -> Address {
        Address { numeric, label }
    }
}

impl From<NonZeroU32> for Address {
    fn from(n: NonZeroU32) -> Self {
        Address::new(Some(n), None)
    }
}

impl From<String> for Address {
    fn from(l: String) -> Self {
        Address::new(None, Some(vec![l]))
    }
}

impl From<&[String]> for Address {
    fn from(ls: &[String]) -> Self {
        Address::new(None, Some(ls.to_vec()))
    }
}

