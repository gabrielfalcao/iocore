//! iocore_test is a testing tool for crates that utilize the
//! [`iocore`] crate.

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

#[macro_export]
macro_rules! current_source_file {
    () => {{
        let mut parts = std::collections::VecDeque::<String>::new();
        for part in file!().split(std::path::MAIN_SEPARATOR_STR) {
            parts.push_back(part.to_string());
        }

        let mut path = std::path::absolute(std::path::Path::new(".")).unwrap();
        while parts.len() > 0 {
            let mut test_path = path.clone();
            for part in &parts {
                test_path.push(part.to_string().as_str());
            }
            if test_path.exists() {
                path = test_path.clone();
                break;
            } else {
                parts.pop_front();
            }
        }

        format!("{}", path.display())
    }}; // () => {{
        //     let parts = module_path!().to_string().split("::").map(String::from).collect::<Vec<String>>();
        //     let mut path = std::path::absolute(std::path::Path::new(".")).unwrap();
        //     for part in parts {
        //         let file = format!("{}.rs",part);
        //         if path.join(&file).exists() {
        //             path.push(&file);
        //             path = path.canonicalize().unwrap();
        //         } else if path.join(&part).exists() {
        //             path.push(&part);
        //             path = path.canonicalize().unwrap();
        //         } else {
        //             break
        //         }
        //     }
        //     format!("{}", path.display())
        // }};
}

/// `path_to_test_file` returns the path to an empty file within the same dir as the calling test file, creates parent directories if necessary and deletes the file if exists
#[macro_export]
macro_rules! path_to_test_file {
    ($name:expr) => {{
        let path = $crate::folder_path!("__test_files__").join($crate::test_name!()).join($name);
        path.parent().unwrap().mkdir().map(|_| ()).unwrap_or_default();
        path.delete().map(|_| ()).unwrap_or_default();
        path
    }};
}

/// `folder_path` returns the path to the parent folder of the calling test file, if called with an argument then calls [`iocore::Path::join`] on the folder path (creates folder if necessary)
#[macro_export]
macro_rules! folder_path {
    () => {{
        let path = iocore::Path::raw($crate::current_source_file!())
            .relative_to_cwd()
            .parent()
            .expect(&format!("{:#?} has no parent folder!!", $crate::current_source_file!()));
        path
    }};
    ($name:expr) => {{
        let path = $crate::folder_path!();
        let path = path.join($name);
        path.mkdir_unchecked();
        path
    }};
}
/// `directory_path` returns the path to the parent directory of the calling test file, if called with an argument then calls [`iocore::Path::join`] on the directory path (creates directory if necessary)
#[macro_export]
macro_rules! directory_path {
    () => {{ $crate::folder_path!() }};
    ($name:expr) => {
        $crate::folder_path!($name)
    };
}

/// `test_folder_parent_path` returns the parent folder of the test file which calls it joined with the given "name" (creates the directory if necessary)
#[macro_export]
macro_rules! test_folder_parent_path {
    ($name:expr) => {{
        let path = iocore::Path::raw($crate::current_source_file!())
            .relative_to_cwd()
            .parent()
            .expect(&format!("parent directory of {:#?}", $crate::current_source_file!()));
        let path = path.join($name);
        path.mkdir_unchecked();
        path
    }};
}
/// `test_directory_parent_path` returns the parent directory of the test file which calls it joined with the given "name" (creates the directory if necessary)
#[macro_export]
macro_rules! test_directory_parent_path {
    ($name:expr) => {{ $crate::test_folder_parent_path($name) }};
}
/// `test_name` returns the name of the test function
#[macro_export]
macro_rules! test_name {
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
    () => {{
        let path = $crate::folder_path!("__test_files__").join($crate::test_name!());
        path.mkdir_unchecked();
        path
    }};
    ($name:expr) => {{
        let path = $crate::path_to_test_folder!().join($name);
        path.mkdir_unchecked();
        path
    }};
}

/// `path_to_test_file` returns the path to a test directory as the test file
#[macro_export]
macro_rules! path_to_test_directory {
    () => {{ $crate::path_to_test_folder!() }};
    ($name:expr) => {{ $crate::path_to_test_folder!($name) }};
}
