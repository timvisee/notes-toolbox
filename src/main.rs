extern crate clap;
use clap::App;

const APP_NAME: &'static str = "NotesToolbox";
const APP_VERSION_NAME: &'static str = "v0.1.0-alpha";
#[allow(dead_code)]
const APP_VERSION_CODE: i32 = 1;

/// Start the application.
fn main() {
    // Configure the application help files, and print if any matches are found
    App::new(APP_NAME).version(APP_VERSION_NAME).get_matches();

    // Show placeholder message
    println!("Hello, world!");
}
