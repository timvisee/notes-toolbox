extern crate clap;
extern crate notes_toolbox;

use clap::App;
use notes_toolbox::util::file::read_file_vec;
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
    let file = read_file_vec(Path::new("./res/testfile.txt"))
        .expect("failed to load file");

    // Print the file length and it's contents (bytes)
    println!("File length: {:?}", file.len());
    println!("File data: {:?}", file);
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
