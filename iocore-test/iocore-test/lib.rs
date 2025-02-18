pub use iocore::*;

pub fn empty_path<D: std::fmt::Display>(
    path: impl Into<Path>,
    name: D,
    extension: Option<D>,
) -> Path {
    let test_file = path.into().with_filename(name.to_string());
    let test_file = match extension {
        Some(s) => test_file.with_extension(s.to_string()),
        None => test_file,
    };

    if test_file.try_canonicalize().exists() {
        match test_file.delete() {
            Ok(_) => {},
            Err(e) => {
                eprintln!("delete {}: {}", &test_file, e);
            },
        }
    }
    match test_file.write(&[]) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("create {}: {}", &test_file, e);
        },
    }
    test_file
}

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

#[macro_export]
macro_rules! path_to_test_file {
    ($name:literal) => {
        iocore_test::empty_path(file!(), $name, Some(&format!("{}", line!())))
    };
    ($name:literal, $extension:literal) => {
        iocore_test::empty_path(file!(), $name, Some($extension))
    };
}

#[macro_export]
macro_rules! folder_path {
    () => {
        iocore::Path::new(file!()).parent().unwrap()
    };
    ($name:literal) => {
        folder_path!().join($name)
    };
}

#[macro_export]
macro_rules! test_folder_parent_path {
    () => {
        match folder_path!().parent() {
            Some(folder) = folder,
            None => {
                panic!("{} has no parent folder!!", folder_path!())
            }
        }
    };
    ($name:literal) => {
        match folder_path!().parent() {
            Some(folder) = folder.join($name),
            None => {
                panic!("{} has no parent folder!!", folder_path!())
            }
        }
    };
}

// #[macro_export]
// macro_rules! scenario {
//     ($name:ident, $test:block) => {
//         #[test]
//         fn test_<$name>() -> ::std::result::Result<(), String> {
//             <$block>
//                 Ok(())
//         }
//     }
// }
