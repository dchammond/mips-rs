#[derive(Clone, Debug)]
pub struct Label {
    pub addr: Option<u32>,
    pub label: String
}

impl Label {
    pub fn new<T,U>(addr: Option<T>, label: U) -> Label where u32: From<T>, String: From<U> {
        Label {addr: match addr { Some(a) => Some(a.into()), None => None }, label: label.into()}
    }
}

