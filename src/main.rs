extern crate clap;
extern crate notes_toolbox;

use clap::App;
use notes_toolbox::app;
use notes_toolbox::util::file::read_file_vec;
use std::path::Path;

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
    App::new(app::APP_NAME)
        .version(app::APP_VERSION_NAME)
        .author(app::APP_AUTHOR)
        .about(app::APP_DESCRIPTION)
        .get_matches();
}
