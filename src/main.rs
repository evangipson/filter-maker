use filter_maker::config::filter::Filter;
use std::fs;

/// [`main`] is the entry point for [`filter_maker`].
fn main() {
    let config = Filter::load_config();
    let _ = fs::write(config.get_destination(), config.get_filter().trim_end());
}
