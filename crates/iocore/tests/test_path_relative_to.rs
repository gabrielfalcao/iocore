use iocore::{Path, Result};
use iocore_test::{
    current_source_file, folder_path, path_to_test_file,
    path_to_test_folder,
};

#[test]
fn test_relative_to_parent_to_child_no_trailing_slash_both_non_existing_paths() -> Result<()> {
    assert_eq!(
        path_to_test_file!("a/b/c/x/y/z.bin")
            .relative_to(&path_to_test_folder!("a/b/c"))
            .to_string(),
        "x/y/z.bin"
    );
    Ok(())
}

#[test]
fn test_relative_to_child_to_parent() -> Result<()> {
    assert_eq!(
        Path::raw("a/b/c/x/y/z.bin").relative_to(&Path::raw("a/b/c")).to_string(),
        "x/y/z.bin"
    );
    assert_eq!(
        Path::raw("a/b/c/x/y/z.bin").relative_to(&Path::raw("a/b/c/")).to_string(),
        "x/y/z.bin"
    );
    Ok(())
}

#[test]
fn test_relative_to_cwd() -> Result<()> {
    assert_eq!(
        Path::cwd().join("iocore/fs/exceptions.rs").relative_to_cwd(),
        Path::raw("iocore/fs/exceptions.rs")
    );
    assert_eq!(
        Path::cwd().join("crates/io/iocore/fs/exceptions.rs").relative_to_cwd(),
        Path::raw("crates/io/iocore/fs/exceptions.rs")
    );
    assert_eq!(
        Path::cwd().join("crates/io/iocore/fs/exceptions.rs").relative_to(&Path::cwd()),
        Path::raw("crates/io/iocore/fs/exceptions.rs")
    );
    assert_eq!(
        Path::cwd().relative_to(&Path::cwd().join("crates/io/iocore/fs/exceptions.rs")),
        Path::raw("../../../../../")
    );
    assert_eq!(
        Path::raw(current_source_file!()).relative_to_cwd(),
        Path::raw("tests/test_path_relative_to.rs")
    );
    Ok(())
}

#[test]
fn test_relative_to_parent_to_child_no_trailing_slash_parent_exists_child_doesnt() -> Result<()> {
    let existing_folder_path = path_to_test_folder!("a/b/c").mkdir()?;
    let nonexisting_file_path = existing_folder_path.join("x/y/z.bin").delete()?;
    assert_eq!(
        existing_folder_path.relative_to(&nonexisting_file_path).to_string(),
        "../../../"
    );
    Ok(())
}

#[test]
fn test_relative_to_parent_to_child_no_trailing_slash_parent_doesnt_exist_child_exists()
-> Result<()> {
    let nonexisting_folder_path = folder_path!(
        "test_relative_to_parent_to_child_no_trailing_slash_parent_doesnt_exist_child_exists/a/b/c"
    )
    .delete()?;
    let existing_file_path = nonexisting_folder_path.join("x/y/z.bin").write(&[])?;
    assert_eq!(
        nonexisting_folder_path.relative_to(&existing_file_path).to_string(),
        "../../../"
    );
    Ok(())
}

#[test]
fn test_relative_to_parent_to_child_no_trailing_slash_both_inexisting_paths() -> Result<()> {
    let test_folder = path_to_test_folder!("a/b/c");
    test_folder.delete_unchecked();
    let test_file = test_folder.join("x/y/z.bin");
    test_file.delete_unchecked();

    assert_eq!(test_folder.relative_to(&test_file).to_string(), "../../../");
    Ok(())
}

#[test]
fn test_relative_to_parent_to_child_with_trailing_slash_both_inexisting_paths() -> Result<()> {
    let test_folder = path_to_test_folder!("a/b/c").delete_unchecked();
    let test_file = path_to_test_file!("a/b/c/x/y/z.bin").delete_unchecked();
    assert_eq!(test_folder.relative_to(&test_file).to_string(), "../../../");
    Ok(())
}

#[test]
fn test_relative_to_parent_to_child_no_trailing_slash_both_existing_paths() -> Result<()> {
    let folder_path = path_to_test_folder!("a/b/c");
    assert_eq!(
        folder_path
            .relative_to(&path_to_test_file!("a/b/c/x/y/z.bin").write_unchecked(&[]))
            .to_string(),
        "../../../"
    );
    assert_eq!(
        path_to_test_folder!("a/b/c")
            .mkdir()?
            .relative_to(&path_to_test_file!("a/b/c/x/y/z.bin").write_unchecked(&[]))
            .to_string(),
        "../../../"
    );
    Ok(())
}
