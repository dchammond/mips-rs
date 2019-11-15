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
