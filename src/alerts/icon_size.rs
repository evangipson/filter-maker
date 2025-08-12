use std::fmt::Display;

/// [`IconSize`] represents a collection of pre-defined visual sizes for alert icons.
#[derive(PartialEq)]
pub enum IconSize {
    /// [`IconSize::Hidden`] will hide a minimap icon.
    Hidden,
    /// [`IconSize::Small`] will display a small icon on the minimap.
    Small,
    /// [`IconSize::Medium`] will display a medium-sized icon on the minimap.
    Medium,
    /// [`IconSize::Large`] will display a large icon on the minimap.
    Large,
}

/// Implement [`Display`] for [`IconSize`].
impl Display for IconSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IconSize::Hidden => "",
                IconSize::Small => "2",
                IconSize::Medium => "1",
                IconSize::Large => "0",
            }
        )
    }
}
