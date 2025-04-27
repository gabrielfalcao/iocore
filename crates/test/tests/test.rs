use iocore::{Path, Result};
use iocore_test::{
    current_source_file, folder_path, path_to_test_file, path_to_test_folder, seq_bytes,
};

#[test]
fn test_path_to_test_file() -> Result<()> {
    let path = path_to_test_file!("test_path_to_test_file").write(&[])?;
    assert!(path.exists());
    assert!(path.is_file());
    Ok(())
}
#[test]
fn test_path_to_test_folder() -> Result<()> {
    path_to_test_folder!("test_path_to_test_folder").mkdir()?;
    assert!(path_to_test_folder!("test_path_to_test_folder").is_directory());
    Ok(())
}

#[test]
fn test_seq_bytes() -> Result<()> {
    assert_eq!(seq_bytes(10), vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    Ok(())
}

#[test]
fn test_folder_path() -> Result<()> {
    assert_eq!(folder_path!(), Path::new(current_source_file!()).parent().unwrap());
    assert_eq!(folder_path!("test.rs"), Path::new(current_source_file!()));
    Ok(())
}
