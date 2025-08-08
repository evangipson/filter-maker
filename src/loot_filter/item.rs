pub struct Item {
    pub base_type: &'static str,
}

impl Item {
    pub const fn new(base_type: &'static str) -> Self {
        Self { base_type }
    }
}
