use filter_maker::config::filter::Filter;
use std::env;
use std::fs;

/// [`main`] is the entry point for [`filter_maker`].
fn main() {
    // grab args from the command line
    let args: Vec<String> = env::args().collect();
    // get the filter path, which is the only argument
    let filter_path = &args[1];
    // load the config using the filter path
    let config = Filter::load_config(filter_path);
    // write the filter
    let _ = fs::write(config.get_destination(), config.get_filter().trim_end());
}
