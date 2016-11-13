extern crate clap;

use clap::App;
use std::env;
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

    // Print all paths to look at when searching for a program
    println!("Program paths:");
    for path in get_program_paths(None) {
        println!(" - {:?}", path);
    }
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

/// Get the PATH variable from the OS environment variables.
///
/// # Examples
///
/// Fetch and print PATH variable to console:
/// ```no_run
/// println!("PATH: {:?}", get_env_path().unwrap());
/// ```
fn get_env_path() -> Option<String> {
    // Get and return the PATH variable
    env::var("PATH").ok()
}

/// Find the paths programs may be installed in on this system.
fn get_program_paths(dir: Option<String>) -> Vec<String> {
    // Create a vector to put the paths in
    let mut paths: Vec<String> = Vec::new();

    // Get the directory string
    let dir_str;
    if dir.is_some() {
        dir_str = dir.unwrap();
    } else {
        dir_str = String::from("");
    }

    // Get the application paths for Windows
    if cfg!(target_os = "windows") {
        // Determine whether we should test the default path
        let mut test_default = true;

        // Create an array of possible program files environment variables for 64-bit systems
        let env_vars = ["ProgramW6432", "ProgramFiles(x86)"];

        // Loop through the variables
        for var_name in env_vars.iter() {
            let prog_path = env::var(var_name);
            if prog_path.is_ok() {
                // Unwrap the path, add it to the vector
                paths.push(
                    Path::new(&prog_path.unwrap())
                        .join(&dir_str)
                        .to_str().unwrap().to_string()
                );

                // Set the test_default flag to false, because we already found a path
                test_default = false;
            }
        }

        // Test the default path, as no path was found yet
        if test_default {
            // Get the default program files path
            let prog_default = env::var("ProgramFiles");
            if prog_default.is_ok() {
                paths.push(
                    Path::new(&prog_default.unwrap())
                        .join(&dir_str)
                        .to_str().unwrap().to_string()
                );
            }
        }
    }

    // Get all paths from the PATH environment variable (cross-platform)
    match get_env_path() {
        Some(env_paths) => {
            for path in env::split_paths(&env_paths) {
                paths.push(path.to_str().unwrap().to_string());
            }
        },
        _ => {}
    }

    // Return the vector of paths
    paths
}
