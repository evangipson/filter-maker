use std::fmt::Display;

/// [`CustomColor`] represents a dynamic color with a red, green, blue, and alpha value.
#[derive(Default, PartialEq)]
pub struct CustomColor {
    /// [`CustomColor::r`] is the red value of a color, from 0-255.
    pub r: u8,
    /// [`CustomColor::g`] is the green value of a color, from 0-255.
    pub g: u8,
    /// [`CustomColor::b`] is the blue value of a color, from 0-255.
    pub b: u8,
    /// [`CustomColor::a`] is the transparency (alpha) value of a color, from 0-255.
    pub a: u8,
}

impl CustomColor {
    /// [`CustomColor::RARE_YELLOW`] is a bright shade of yellow usually used to denote rare items.
    pub const RARE_YELLOW: CustomColor = CustomColor::new(255, 255, 119, 255);

    /// [`CustomColor::DIVINE_RED`] is a bright shade of red usually used to denote high-value items.
    pub const DIVINE_RED: CustomColor = CustomColor::new(191, 0, 0, 255);

    /// [`CustomColor::QUEST_GREEN`] is a bright shade of green usually used to denote quest items.
    pub const QUEST_GREEN: CustomColor = CustomColor::new(27, 119, 65, 255);

    /// [`CustomColor::CURRENCY_SALMON`] is a medium shade of pink used to denote currency items.
    pub const CURRENCY_SALMON: CustomColor = CustomColor::new(254, 128, 128, 255);

    /// [`CustomColor::DARK_PINK`] is a dark shade of pink.
    pub const DARK_PINK: CustomColor = CustomColor::new(211, 84, 25, 255);

    /// [`CustomColor::PURPLE`] is a dark shade of purple.
    pub const PURPLE: CustomColor = CustomColor::new(142, 68, 173, 255);

    /// [`CustomColor::MAGIC_BLUE`] is a bright shade of blue usually used to denote magic items.
    pub const MAGIC_BLUE: CustomColor = CustomColor::new(136, 136, 255, 255);

    /// [`CustomColor::DARK_MAGIC_BLUE`] is a dark shade of blue usually used to denote magic items.
    pub const DARK_MAGIC_BLUE: CustomColor = CustomColor::new(41, 128, 185, 255);

    /// [`CustomColor::UNIQUE_ORANGE`] is a bright shade of orange usually used to denote unique items.
    pub const UNIQUE_ORANGE: CustomColor = CustomColor::new(175, 96, 37, 255);

    /// [`CustomColor::KALGUUR_GOLD`] is a bright shade of gold usually used to denote gold.
    pub const KALGUUR_GOLD: CustomColor = CustomColor::new(187, 165, 61, 255);

    /// [`CustomColor::GEM_TEAL`] is a bright shade of teal usually used to denote skill and support gems.
    pub const GEM_TEAL: CustomColor = CustomColor::new(7, 122, 115, 255);

    /// [`CustomColor::NORMAL_WHITE`] is a bright shade of white usually used for readable text.
    pub const NORMAL_WHITE: CustomColor = CustomColor::new(200, 200, 200, 255);

    /// [`CustomColor::BLACK`] is a dark shade of black usually used as a background for readable text.
    pub const BLACK: CustomColor = CustomColor::new(42, 42, 42, 255);

    /// [`CustomColor::FADED_BLACK`] is a dark shade of translucent black used as a non-invasive background for readable text.
    pub const FADED_BLACK: CustomColor = CustomColor::new(42, 42, 42, 150);

    /// [`CustomColor::TRANSPARENT`] represents complete transparency (or no color).
    pub const TRANSPARENT: CustomColor = CustomColor::new(0, 0, 0, 0);

    /// [`CustomColor::new`] will create a new [`CustomColor`].
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// Implement [`Display`] for [`CustomColor`].
impl Display for CustomColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {} {}", self.r, self.g, self.b, self.a)
    }
}
