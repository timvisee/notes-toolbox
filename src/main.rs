extern crate clap;
#[cfg(test)]
extern crate tempdir;

#[cfg(test)]
pub mod fs_util;

use clap::App;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const APP_NAME: &'static str = "NotesToolbox";
const APP_VERSION_NAME: &'static str = "v0.1.0-alpha";
#[allow(dead_code)]
const APP_VERSION_CODE: i32 = 1;
const APP_AUTHOR: &'static str = "Tim Visee <timvisee@gmail.com>";
const APP_DESCRIPTION: &'static str = "Toolbox project for compiling notes into PDF's, slides and \
        some other formats!";
#[cfg(windows)]
#[allow(dead_code)]
const ENV_PATH_DELIMITER: &'static str = ";";
#[cfg(not(windows))]
#[allow(dead_code)]
const ENV_PATH_DELIMITER: &'static str = ":";

/// Start the application.
fn main() {
    // Handle command line arguments for help
    handle_arguments();

    // Load a file
    let file = load_file_vec(Path::new("./res/testfile.txt"))
        .expect("failed to load file");

    // Print the file length and it's contents (bytes)
    println!("File length: {:?}", file.len());
    println!("File data: {:?}", file);
}

/// Load the given file as vector, where `path` is the path the file is loaded from.
///
/// # Examples
///
/// Load a file, and print it's file lines and raw bytes:
/// ```no_run
/// let file = load_file(Path::new("~/myfile"))
///         .expect("failed to load file");
///
/// println!("File length: {:?}", file.len());
/// println!("File data: {:?}", file);
/// ```
fn load_file_vec(path: &Path) -> Result<Vec<u8>, &'static str> {
    // Try to open the file
    let file_result = File::open(path);

    // Handle errors
    if file_result.is_err() {
        return Err("failed to open file");
    }

    // Get the actual file, and create a new data buffer
    let mut file = file_result.unwrap();
    let mut data = Vec::new();

    // Read the actual file
    if file.read_to_end(&mut data).is_err() {
        return Err("failed to read file");
    }

    // Return the data vector containing the loaded file
    Ok(data)
}

/// Handle program arguments passed along with the command line to show things like help pages.
///
/// # Examples
///
/// Parse application parameters, and show relevant information such as help files in the console:
/// ```no_run
/// handle_arguments();
/// ```
fn handle_arguments() {
    // Configure the application object with help information, and show matches
    App::new(APP_NAME)
        .version(APP_VERSION_NAME)
        .author(APP_AUTHOR)
        .about(APP_DESCRIPTION)
        .get_matches();
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use tempdir::TempDir;

    use super::load_file_vec;
    use fs_util::remove_dir_all_force;

    #[test]
    fn load_file_vec_test() {
        // Create a new vector
        let base_vec = vec![72u8, 101u8, 108u8, 108u8, 111u8, 44u8, 32u8, 87u8, 111u8, 114u8, 108u8,
                100u8, 33u8]; // Hello, World!

        // Create a temporary directory for testing
        let temp_dir = TempDir::new("notes_toolbox")
                .expect("failed to create temporary notes_toolbox directory");

        // Determine the path for the test file
        let file_path = temp_dir.path().join("test_file.txt");

        // Create and write the file
        let mut file = File::create(&file_path).expect("failed to create temporary file");
        file.write_all(base_vec.as_slice()).expect("failed to write to temporary file");
        file.sync_all().unwrap();

        // Load the vector
        let file_vec = load_file_vec(&file_path).expect("failed to read file to vector");

        // Make sure the read vector is equal to the base vector
        assert_eq!(file_vec, base_vec, "loaded file vector is different than base vector");

        // Remove the temporary directory
        remove_dir_all_force(temp_dir.into_path());
    }
}