use crate::constants::filter;
use std::{fs, path::Path};

/// [`copy_filter`] will copy the generated filter to `destination_path`.
/// # Example
/// You can use [`copy_filter`] to copy a generated filter to any path:
/// ```rust
/// use filter_maker::os::copy_file;
///
/// fn do_filter_copy() {
///     copy_file::copy_filter("C:/Temp");
/// }
/// ```
pub fn copy_filter(destination_path: &str) {
    let _ = fs::copy(
        format!("./{}", filter::FILTER_NAME),
        format!(
            "{}/{}",
            Path::new(destination_path).to_str().unwrap(),
            filter::FILTER_NAME
        ),
    );
}
