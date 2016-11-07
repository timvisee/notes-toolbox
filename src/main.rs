extern crate clap;

use clap::App;
use std::process::Command;
use std::env;

const APP_NAME: &'static str = "NotesToolbox";
const APP_VERSION_NAME: &'static str = "v0.1.0-alpha";
#[allow(dead_code)]
const APP_VERSION_CODE: i32 = 1;
const APP_AUTHOR: &'static str = "Tim Visee <timvisee@gmail.com>";
const APP_DESCRIPTION: &'static str = "Toolbox project for compiling notes into PDF's, slides and \
        some other formats!";

/// Start the application.
#[allow(unused_variables)]
fn main() {
    // Handle command line arguments for help
    handle_arguments();

    // Show placeholder message
    println!("Hello, world!");

    // Get the environment PATH variable and print it to the console
    println!("PATH: {:?}", get_env_path().unwrap());
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

/// Get the PATH variable from the OS environment variables.
///
/// # Examples
///
/// Fetch and print PATH variable to console:
/// ```rust
/// println!("PATH: {:?}", get_env_path().unwrap());
/// ```
fn get_env_path() -> Option<String> {
    // Iterate through the environment variables of the OS
    for (key, value) in env::vars_os() {
        // Get the OsString (key) as a string
        let key_str = key.into_string().expect("Failed to convert OsString into String for environment key");

        // Continue the loop if this isn't the PATH variable
        if key_str != "PATH".to_string() {
            continue;
        }

        // Variable found, return the option with it's value
        return Some(value.into_string().expect("Failed to parse environment variable value to String"));
    }

    // Variable not found, return nothing
    None
}