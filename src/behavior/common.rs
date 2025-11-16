use crate::{color::custom_color::CustomColor, config::color::Color};

/// [`get_color`] will get a color based on a `palette` and `name`.
/// # Example
/// [`get_color`] can be used to get a color `palette` based on a `name`:
/// ```rust
/// use filter_maker::{
///     color::custom_color::CustomColor,
///     config::color::Color,
///     behavior::common
/// };
///
/// fn get_red_color(palette: &[Color]) -> CustomColor {
///     common::get_color(palette, "red")
/// }
/// ```
pub fn get_color(palette: &[Color], name: &str) -> CustomColor {
    if !name.is_empty() {
        palette
            .iter()
            .find(|p| p.name.clone().unwrap_or_default().to_lowercase() == name.to_lowercase())
            .cloned()
            .unwrap_or_default()
            .color
            .unwrap_or(CustomColor::TRANSPARENT)
    } else {
        CustomColor::TRANSPARENT
    }
}

/// [`capitalize`] will capitlize `string_to_capitalize`.
/// # Example
/// [`capitalize`] can be used to capitlize any [`str`]:
/// ```rust
/// use filter_maker::behavior::common;
///
/// fn capitalize_string(string: &str) -> String {
///     common::capitalize(string)
/// }
/// ```
pub fn capitalize(string_to_capitlize: &str) -> String {
    if string_to_capitlize.len() > 1 {
        string_to_capitlize[0..1].to_uppercase() + &string_to_capitlize[1..]
    } else {
        string_to_capitlize.to_uppercase()
    }
}
