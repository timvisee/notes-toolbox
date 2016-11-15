use std::fs;
use std::path::PathBuf;

/// Force remove a directory structure (including all of it's contents).
/// The regular `fs::remove_dir_all(...)` function might inconsistently fail on Windows machines.
/// This function forces to delete the directory, by repeatedly invoking the remove command until
/// the file is successfully removed. The remove command is attempted a maximum number of times to
/// prevent locking.
/// Calls to this function on non-Windows machines automatically use `fs::remove_dir_all(...)`.
///
/// True is returned if the directory was successfully removed. False is returned if the directory
/// structure couldn't be removed, and the maximum number of remove attempts is reached.
///
/// # Example
///
/// Force remove a directory structure on Windows:
/// ```no_run
/// remove_dir_all_force(Path::new("~/myfile");
/// ```
pub fn remove_dir_all_force(path: PathBuf) -> bool {
    // Remove directories normally non-Windows
    if !cfg!(target_family = "windows") {
        return fs::remove_dir_all(path).is_ok();
    }

    // Create a variable to define the number of attempts we've left
    // TODO: Don't hardcode this value in the function itself, create a constant?
    let mut attempt: u8 = 8;

    // Keep trying to remove the directory until attempts are drained
    while attempt > 0 {
        // Try to remove the directory, and return on success
        if fs::remove_dir_all(&path).is_ok() {
            return true;
        }

        // Decrease the number of attempts
        attempt -= 1;
    }

    // We failed
    false
}
