use std::fmt::Display;

/// [`IconType`] represents a collection of pre-defined icon shapes for alerts.
#[derive(PartialEq)]
pub enum IconType {
    /// [`IconType::None`] will hide an icon on the minimap.
    None,
    /// [`IconType::Circle`] will display a circle icon on the minimap.
    Circle,
    /// [`IconType::Diamond`] will display a diamond icon on the minimap.
    Diamond,
    /// [`IconType::Hexagon`] will display a hexagon icon on the minimap.
    Hexagon,
    /// [`IconType::Square`] will display a square icon on the minimap.
    Square,
    /// [`IconType::Star`] will display a star icon on the minimap.
    Star,
    /// [`IconType::Triangle`] will display a triangle icon on the minimap.
    Triangle,
    /// [`IconType::Cross`] will display a cross icon on the minimap.
    Cross,
    /// [`IconType::Moon`] will display a moon icon on the minimap.
    Moon,
    /// [`IconType::Raindrop`] will display a rain drop icon on the minimap.
    Raindrop,
    /// [`IconType::Kite`] will display a kite icon on the minimap.
    Kite,
    /// [`IconType::Pentagon`] will display a pentagon icon on the minimap.
    Pentagon,
    /// [`IconType::UpsideDownHouse`] will display an upside-down house icon on the minimap.
    UpsideDownHouse,
}

/// Implement [`Display`] for [`IconType`].
impl Display for IconType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IconType::None => "",
                IconType::Circle => "Circle",
                IconType::Diamond => "Diamond",
                IconType::Hexagon => "Hexagon",
                IconType::Square => "Square",
                IconType::Star => "Star",
                IconType::Triangle => "Triangle",
                IconType::Cross => "Cross",
                IconType::Moon => "Moon",
                IconType::Raindrop => "Raindrop",
                IconType::Kite => "Kite",
                IconType::Pentagon => "Pentagon",
                IconType::UpsideDownHouse => "UpsideDownHouse",
            }
        )
    }
}
