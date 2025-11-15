use std::{fs, path::Path};

/// [`copy_filter`] will copy the generated filter to `destination_path`.
pub fn copy_filter(destination_path: &str) {
    let _ = fs::copy(
        "./FilterMaker.filter",
        format!(
            "{}/FilterMaker.filter",
            Path::new(destination_path).to_str().unwrap()
        ),
    );
}
