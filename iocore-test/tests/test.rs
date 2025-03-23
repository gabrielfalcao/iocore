use iocore::{Path, Result};
use iocore_test::{folder_path, path_to_test_file, seq_bytes};

#[test]
fn test_path_to_test_file() -> Result<()> {
    path_to_test_file!("test_path_to_test_file").write(&[])?;
    assert_eq!(
        path_to_test_file!("test_path_to_test_file.6"),
        Path::new(file!()).with_filename("test_path_to_test_file.6")
    );
    assert!(path_to_test_file!("test_path_to_test_file").exists());
    assert_eq!(
        path_to_test_file!("test_path_to_test_file", "test"),
        Path::new(file!()).with_filename("test_path_to_test_file.test")
    );
    Ok(())
}

#[test]
fn test_seq_bytes() -> Result<()> {
    assert_eq!(seq_bytes(10), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    Ok(())
}

#[test]
fn test_folder_path() -> Result<()> {
    assert_eq!(folder_path!(), Path::new(file!()).parent().unwrap());
    assert_eq!(folder_path!("test.rs"), Path::new(file!()));
    Ok(())
}
