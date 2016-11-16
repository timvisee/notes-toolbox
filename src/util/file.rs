#[cfg(test)]
extern crate tempdir;

use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Load the given file and read it as a vector, where `path` is the path the file is loaded from.
///
/// # Examples
///
/// Load a file, and print it's file lines and raw bytes:
/// ```no_run
/// let file = read_file_vec(Path::new("~/my_file"))
///         .expect("failed to read file");
///
/// println!("File length: {:?}", file.len());
/// println!("File data: {:?}", file);
/// ```
pub fn read_file_vec(path: &Path) -> Result<Vec<u8>, &'static str> {
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

    // Return the data vector containing the read file
    Ok(data)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use util::fs::remove_dir_all_force;

    use super::tempdir::TempDir;
    use super::read_file_vec;

    #[test]
    fn read_file_vec_test() {
        // Create a new vector
        let base_vec = vec![72u8, 101u8, 108u8, 108u8, 111u8, 44u8, 32u8, 87u8, 111u8, 114u8, 108u8,
        100u8, 33u8]; // Hello, World!

        // Create a temporary directory for testing
        let temp_dir = TempDir::new("notes_toolbox")
            .expect("failed to create temporary notes_toolbox directory");

        // Determine the path for the test file
        let file_path = temp_dir.path().join("my_file.txt");

        // Create and write the file
        let mut file = File::create(&file_path).expect("failed to create temporary file");
        file.write_all(base_vec.as_slice()).expect("failed to write to temporary file");
        file.sync_all().unwrap();

        // Read the file
        let file_vec = read_file_vec(&file_path).expect("failed to read file to vector");

        // Make sure the read vector is equal to the base vector
        assert_eq!(file_vec, base_vec, "loaded file vector is different than base vector");

        // Remove the temporary directory
        remove_dir_all_force(temp_dir.into_path());
    }
}
