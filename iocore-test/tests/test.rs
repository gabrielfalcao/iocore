use iocore_test::{dir_path, path_to_test_file, seq_bytes, Path};

#[test]
fn test_path_to_test_file() {
    assert_eq!(
        path_to_test_file!("test_path_to_test_file"),
        Path::new(file!()).with_filename("test_path_to_test_file.6")
    );
    assert!(path_to_test_file!("test_path_to_test_file").exists());
    assert_eq!(
        path_to_test_file!("test_path_to_test_file", "test"),
        Path::new(file!()).with_filename("test_path_to_test_file.test")
    );
}

#[test]
fn test_seq_bytes() {
    assert_eq!(seq_bytes(10), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn test_dir_path() {
    assert_eq!(dir_path!(), Path::new(file!()).parent().unwrap());
    assert_eq!(dir_path!("test.rs"), Path::new(file!()));
}
