extern crate clap;

use clap::App;

const APP_NAME: &'static str = "NotesToolbox";
const APP_VERSION_NAME: &'static str = "v0.1.0-alpha";
#[allow(dead_code)]
const APP_VERSION_CODE: i32 = 1;
const APP_AUTHOR: &'static str = "Tim Visee <timvisee@gmail.com>";
const APP_DESCRIPTION: &'static str = "Toolbox project for compiling notes into PDF's, slides and \
        some other formats!";

/// Start the application.
fn main() {
    // Handle command line arguments for help
    handle_arguments();

    // Show placeholder message
    println!("Hello, world!");
}

/// Handle program arguments passed along with the command line to show things like help pages.
fn handle_arguments() {
    // Configure the application object with help information, and show matches
    App::new(APP_NAME)
        .version(APP_VERSION_NAME)
        .author(APP_AUTHOR)
        .about(APP_DESCRIPTION)
        .get_matches();
}
