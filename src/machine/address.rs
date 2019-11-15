use std::num::NonZeroU32;

#[derive(Clone, Debug)]
pub struct Address {
    pub numeric: Option<NonZeroU32>,
    pub label: Option<String>,
}

impl Address {
    pub fn new(numeric: Option<NonZeroU32>, label: Option<String>) -> Address {
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
        Address::new(None, Some(l))
    }
}

