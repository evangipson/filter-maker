use std::{
    fs::{self, DirEntry},
    io::Error,
    path::{Path, PathBuf},
};

/// [`find_folder_in_directory`] finds a folder that matches `target_folder_name` in `directory`,
/// and returns either a [`PathBuf`], or an [`Error`] if it isn't found.
/// # Example
/// [`find_folder_in_directory`] can be used to find a folder by name in a directory:
/// ```rust
/// use filter_maker::os::find_directory;
/// use std::{io::Error, path::{Path, PathBuf}};
///
/// fn find_temp_folder() -> Result<PathBuf, Error> {
///     let directory = Path::new("C:/");
///     find_directory::find_folder_in_directory(&directory, "Temp")
/// }
/// ```
pub fn find_folder_in_directory(
    directory: &Path,
    target_folder_name: &str,
) -> Result<PathBuf, Error> {
    // find all folders in `directory` that match `target_folder_name`
    let target_directory_folders: Vec<Result<DirEntry, Error>> = fs::read_dir(directory)
        .unwrap()
        .filter(|d| {
            let dir_entry = d.as_ref().unwrap().path();
            dir_entry.is_dir() && dir_entry.to_str().unwrap().contains(target_folder_name)
        })
        .collect();

    // give back the first match, or an error
    if let Some(Ok(target_folder)) = target_directory_folders.first() {
        Ok(target_folder.path())
    } else {
        Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find any directory.",
        ))
    }
}

/// [`find_filter_destination`] will do it's best to find the installation directory for Path of Exile,
/// and returns either a [`PathBuf`], or an [`Error`] if it isn't found.
/// # Example
/// [`find_filter_destination`] can be used to find the most appropriate filter destination:
/// ```rust
/// use filter_maker::os::find_directory;
/// use std::{io::Error, path::{Path, PathBuf}};
///
/// fn find_poe_installation_folder() -> Result<PathBuf, Error> {
///     find_directory::find_filter_destination()
/// }
/// ```
pub fn find_filter_destination() -> Result<PathBuf, Error> {
    // get the home directory
    let home_dir = std::env::home_dir().unwrap();

    // if there is a ~/OneDrive/ folder, favor that
    let next_dir = if let Ok(onedrive_dir) = find_folder_in_directory(&home_dir, "OneDrive") {
        onedrive_dir
    } else {
        home_dir
    };

    // look for the Documents folder, either off of ~/ or ~/OneDrive/
    let documents_dir = find_folder_in_directory(Path::new(&next_dir), "Documents");
    if documents_dir.is_err() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Could not even find a Documents directory bro.",
        ));
    }

    // look for the My Games folder, off of ~/Documents/, or ~/OneDrive/Documents/
    let my_games_dir = find_folder_in_directory(Path::new(&documents_dir.unwrap()), "My Games");
    if my_games_dir.is_err() {
        return Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find the My Games directory, idiot.",
        ));
    }

    // look for the Path of Exile folder in the My Games folder
    let poe_dir = find_folder_in_directory(Path::new(&my_games_dir.unwrap()), "Path of Exile");
    if let Ok(poe_directory) = poe_dir {
        Ok(poe_directory)
    } else {
        Err(Error::new(
            std::io::ErrorKind::NotFound,
            "Could not find Path of Exile directory in ~/Documents",
        ))
    }
}
