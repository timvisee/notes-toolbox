//extern crate tempdir;

//use std::fs::File;
//use std::io::Write;
//use std::path::Path;
//use tempdir::TempDir;

#[test]
fn sum_test() {
    assert_eq!(2 + 3, 5);
}

//#[allow(dead_code)]
//fn load_file_vec_test() {
//    // Create a new vector
//    let base_vec = vec![60, 61, 62];
//
//    if let Ok(dir) = TempDir::new("notes_toolbox") {
//        let file_path = dir.path().join("testfile.txt");
//        println!("Path: {:?}", file_path);
//
//        let mut f = try!(File::create(file_path));
//        try!(f.write_all(b"Hello, world!"));
//        try!(f.sync_all());
//        try!(dir.close());
//    }
//
//    // TODO: Read the file, and make sure the contents are read properly!
//}
