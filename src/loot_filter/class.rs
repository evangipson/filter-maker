use std::fmt::Display;

pub struct Class {
    pub name: &'static str,
}

impl Class {
    pub const CURRENCY: Class = Class::new("Currency");

    pub const fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
