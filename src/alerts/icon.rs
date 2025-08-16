use crate::alerts::{color::Color, icon_size::IconSize, icon_type::IconType};
use std::fmt::Display;

/// [`AlertIcon`] represents an icon on the minimap.
#[derive(PartialEq)]
pub struct AlertIcon {
    /// [`AlertIcon::size`] represents the size of the icon on the minimap.
    pub size: IconSize,
    /// [`AlertIcon::color`] represents the color of the icon on the minimap.
    pub color: Color,
    /// [`AlertIcon::icon`] represents the shape of the icon on the minimap.
    pub icon: IconType,
}

/// Implement [`AlertIcon`].
impl AlertIcon {
    /// [`AlertIcon::NONE`] will hide an icon on the minimap.
    pub const NONE: AlertIcon = AlertIcon::new(IconSize::Hidden, Color::None, IconType::None);

    /// [`AlertIcon::SMALL_YELLOW_CIRCLE`] will show a small yellow circle icon on the minimap.
    pub const SMALL_YELLOW_CIRCLE: AlertIcon =
        AlertIcon::new(IconSize::Small, Color::Yellow, IconType::Circle);

    /// [`AlertIcon::BIG_RED_STAR`] will show a big red star icon on the minimap.
    pub const BIG_RED_STAR: AlertIcon = AlertIcon::new(IconSize::Large, Color::Red, IconType::Star);

    /// [`AlertIcon::YELLOW_STAR`] will show a medium-sized yellow star icon on the minimap.
    pub const YELLOW_STAR: AlertIcon =
        AlertIcon::new(IconSize::Medium, Color::Yellow, IconType::Star);

    /// [`AlertIcon::PURPLE_KITE`] will show a medium-sized purple kite icon on the minimap.
    pub const PURPLE_KITE: AlertIcon =
        AlertIcon::new(IconSize::Medium, Color::Purple, IconType::Kite);

    /// [`AlertIcon::SMALL_RED_CIRCLE`] will show a small red circle icon on the minimap.
    pub const SMALL_RED_CIRCLE: AlertIcon =
        AlertIcon::new(IconSize::Small, Color::Red, IconType::Circle);

    /// [`AlertIcon::SMALL_WHITE_CIRCLE`] will show a small white circle icon on the minimap.
    pub const SMALL_WHITE_CIRCLE: AlertIcon =
        AlertIcon::new(IconSize::Small, Color::White, IconType::Circle);

    /// [`AlertIcon::PINK_MOON`] will show a medium-sized pink moon icon on the minimap.
    pub const PINK_MOON: AlertIcon = AlertIcon::new(IconSize::Medium, Color::Pink, IconType::Moon);

    /// [`AlertIcon::new`] will create a new [`AlertIcon`].
    const fn new(size: IconSize, color: Color, icon: IconType) -> Self {
        Self { size, color, icon }
    }
}

/// Implement [`Display`] for [`AlertIcon`].
impl Display for AlertIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alert_icon = if self.size == IconSize::Hidden
            || self.color == Color::None
            || self.icon == IconType::None
        {
            String::new()
        } else {
            format!("MinimapIcon {} {} {}", self.size, self.color, self.icon)
        };

        write!(f, "{alert_icon}")
    }
}
