//! iocore_test is a testing tool for crates that utilize the
//! [`iocore`] crate.

use iocore::*;

/// `empty_path` returns [`iocore::Path`] deletes path if exists, but creates parent paths
pub fn empty_path(
    path: impl Into<Path>,
    name: impl std::fmt::Display,
    extension: Option<impl std::fmt::Display>,
) -> Path {
    let test_file = path.into().with_filename(name.to_string());
    let test_file = match extension {
        Some(s) => test_file.with_extension(s.to_string()),
        None => test_file,
    };
    test_file.mkdir_parents().map(|_| ()).unwrap_or_default();
    if test_file.exists() {
        match test_file.delete() {
            Ok(_) => {},
            Err(_) => {},
        }
    }
    test_file
}

/// `seq_bytes` returns a [`Vec<u8>`] containing a sequence of [`u8`]
/// bytes and applying the remainder operation if `count` is longer
/// than `u8::MAX`
pub fn seq_bytes(count: usize) -> Vec<u8> {
    (0..count)
        .map(|n| {
            TryInto::<u8>::try_into(if n > u8::MAX.into() {
                n % <u8 as Into<usize>>::into(u8::MAX)
            } else {
                n
            })
            .unwrap()
        })
        .collect()
}

/// `path_to_test_file` returns the path to an empty file within the same dir as the calling test file, creates parent directories if necessary and deletes the file if exists
#[macro_export]
macro_rules! path_to_test_file {
    ($name:expr) => {{
        let path = folder_path!().join($crate::test_function_name!()).join($name);
        path.mkdir_parents().map(|_| ()).unwrap_or_default();
        path.delete().map(|_| ()).unwrap_or_default();
        path
    }};
    ($name:expr, $extension:expr) => {
        iocore_test::empty_path($crate::test_function_name!(), $name, Some($extension))
    };
}

/// `folder_path` returns the path to the parent folder of the calling test file, if called with an argument then calls [`iocore::Path::join`] on the folder path (creates folder if necessary)
#[macro_export]
macro_rules! folder_path {
    () => {{
        let path = Path::raw(file!())
            .parent()
            .expect(&format!("{:#?} has no parent folder!!", file!()));
        path
    }};
    ($name:expr) => {{
        let path = folder_path!();
        let path = path.join($name);
        path.mkdir_unchecked();
        path
    }};
}

/// `test_folder_parent_path` returns the parent folder of the test file which calls it joined with the given "name" (creates the directory if necessary)
#[macro_export]
macro_rules! test_folder_parent_path {
    ($name:expr) => {{
        let path = Path::raw(file!())
            .parent()
            .expect(&format!("parent directory of {:#?}", file!()));
        let path = path.join($name);
        path.mkdir_unchecked();
        path
    }};
}
/// `test_function_name` returns the name of the test function
#[macro_export]
macro_rules! test_function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let name = name.strip_suffix("::f").unwrap().replace("::", std::path::MAIN_SEPARATOR_STR);
        name
    }};
}

/// `path_to_test_file` returns the path to a test directory as the test file
#[macro_export]
macro_rules! path_to_test_folder {
    ($name:expr) => {{
        let path = path_to_test_file!($name);
        path.mkdir_unchecked();
        path
    }};
}
