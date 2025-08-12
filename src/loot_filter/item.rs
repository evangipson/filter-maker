/// [`Item`] represents a specific item.
#[derive(PartialEq, Clone, Copy)]
pub struct Item {
    /// [`Item::base_type`] is the name of an item.
    pub base_type: &'static str,
}

/// Implement [`Item`].
impl Item {
    /// [`Item::new`] will create a new [`Item`].
    pub const fn new(base_type: &'static str) -> Self {
        Self { base_type }
    }
}
