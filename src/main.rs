use filter_maker::{
    config::filter::Filter,
    os::{copy_file, find_directory},
};
use std::env;
use std::fs;
use std::io::Error;

/// [`main`] is the entry point for [`filter_maker`].
fn main() -> Result<(), Error> {
    // get the filter path command line argument, which is the only required argument
    let filter_path = &env::args()
        .nth(1)
        .expect("Must provide TOML configuration file path.");

    // get the destination path command line argument, if it was provided
    let destination_path = &env::args().nth(2).unwrap_or_default();

    // load the config using the filter path
    let config = Filter::load_config(filter_path);

    // write the filter
    let _ = fs::write("FilterMaker.filter", config.get_filter().trim_end());

    // find the best place to copy the filter, then copy it
    if !destination_path.is_empty() {
        copy_file::copy_filter(destination_path);
        Ok(())
    } else if let Ok(filter_destination) = find_directory::find_filter_destination() {
        copy_file::copy_filter(filter_destination.as_path().to_str().unwrap());
        Ok(())
    } else {
        Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find a destination to copy the filter to.",
        ))
    }
}
