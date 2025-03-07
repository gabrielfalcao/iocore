//! iocore_test is a testing tool for crates that utilize the
//! [`iocore`] crate.

use iocore::*;

/// `empty_path` returns [`iocore::Path`] deletes path if exists
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

    if test_file.exists() {
        match test_file.delete() {
            Ok(_) => {},
            Err(e) => {
                eprintln!("delete {}: {}", &test_file, e);
            },
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

/// `path_to_test_file` returns the path to an empty file within the same dir as the test file which calls it
#[macro_export]
macro_rules! path_to_test_file {
    ($name:literal) => {
        iocore_test::empty_path(file!(), $name, iocore::Path::new($name).extension())
    };
    ($name:literal, $extension:literal) => {
        iocore_test::empty_path(file!(), $name, Some($extension))
    };
}

/// `folder_path` returns the path to the test file which calls it
#[macro_export]
macro_rules! folder_path {
    () => {
        match iocore::Path::new(file!()).parent() {
            Some(folder) => folder,
            None => {
                panic!("{} has no parent folder!!", file!()) // preposterous but gracefully covered
            },
        }
    };
    ($name:literal) => {
        match iocore::Path::new(file!()).parent() {
            Some(folder) => folder.join($name),
            None => {
                panic!("{} has no parent folder!!", file!()) // preposterous but gracefully covered
            },
        }
    };
}

/// `test_folder_parent_path` returns the parent folder of the test file which calls it
#[macro_export]
macro_rules! test_folder_parent_path {
    () => {
        {
            let path = folder_path!();
            match path.parent() {
                Some(folder) = folder,
                None => {
                    panic!("{} has no parent folder!!", &path)
                }
            }
        }
    };
    ($name:literal) => {
        {
            let path = folder_path!();
            match path.parent() {
                Some(folder) = folder.join($name),
                None => {
                    panic!("{} has no parent folder!!", &path)
                }
            }
        }
    };
}
