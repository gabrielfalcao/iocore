#[cfg(test)]
mod test_very_specific_ordering_files {
    use iocore::Path;
    use iocore_test::path_to_test_file;
    #[test]
    fn test_paths_should_be_ordered_alphabetically() {
        let mut paths = vec![
            path_to_test_file!("zzzzz"),
            path_to_test_file!("mmmmm"),
            path_to_test_file!("nnnnn"),
            path_to_test_file!("aaaaa"),
        ];
        paths.sort();
        assert_eq!(
            paths,
            vec![
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files/test_paths_should_be_ordered_alphabetically/aaaaa"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files/test_paths_should_be_ordered_alphabetically/mmmmm"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files/test_paths_should_be_ordered_alphabetically/nnnnn"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files/test_paths_should_be_ordered_alphabetically/zzzzz"),
            ]
        );
    }
    #[test]
    fn but_paths_should_be_ordered_by_length() {
        let mut paths = vec![
            path_to_test_file!("mmmmm"),
            path_to_test_file!("mmm"),
            path_to_test_file!("aaa"),
            path_to_test_file!("aaaa"),
        ];
        paths.sort();
        assert_eq!(
            paths,
            vec![
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files/but_paths_should_be_ordered_by_length/aaa"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files/but_paths_should_be_ordered_by_length/aaaa"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files/but_paths_should_be_ordered_by_length/mmm"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files/but_paths_should_be_ordered_by_length/mmmmm"),
            ]
        );
    }
    #[test]
    fn but_paths_should_be_ordered_by_depth_of_folders() {
        let mut paths = vec![
            path_to_test_file!("abcdefg"),
            path_to_test_file!("nopqrst"),
            path_to_test_file!("a/bcdefg"),
            path_to_test_file!("no/pqrst"),
            path_to_test_file!("u/v/w/x/y"),
            path_to_test_file!("uv/wx/y"),
        ];
        paths.sort();
        assert_eq!(
            paths,
            vec![
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files/but_paths_should_be_ordered_by_depth_of_folders/abcdefg"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files/but_paths_should_be_ordered_by_depth_of_folders/nopqrst"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files/but_paths_should_be_ordered_by_depth_of_folders/a/bcdefg"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files/but_paths_should_be_ordered_by_depth_of_folders/no/pqrst"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files/but_paths_should_be_ordered_by_depth_of_folders/uv/wx/y"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files/but_paths_should_be_ordered_by_depth_of_folders/u/v/w/x/y"),
            ]
        );
    }
}
#[cfg(test)]
mod test_very_specific_ordering_folders {
    use iocore::Path;
    use iocore_test::path_to_test_folder;
    #[test]
    fn test_paths_should_be_ordered_alphabetically() {
        let mut paths = vec![
            path_to_test_folder!("zzzzz"),
            path_to_test_folder!("mmmmm"),
            path_to_test_folder!("nnnnn"),
            path_to_test_folder!("aaaaa"),
        ];
        paths.sort();
        assert_eq!(
            paths,
            vec![
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_folders/test_paths_should_be_ordered_alphabetically/aaaaa"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_folders/test_paths_should_be_ordered_alphabetically/mmmmm"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_folders/test_paths_should_be_ordered_alphabetically/nnnnn"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_folders/test_paths_should_be_ordered_alphabetically/zzzzz"),
            ]
        );
    }
    #[test]
    fn but_paths_should_be_ordered_by_length() {
        let mut paths = vec![
            path_to_test_folder!("mmmmm"),
            path_to_test_folder!("mmm"),
            path_to_test_folder!("aaa"),
            path_to_test_folder!("aaaa"),
        ];
        paths.sort();
        assert_eq!(
            paths,
            vec![
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_folders/but_paths_should_be_ordered_by_length/aaa"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_folders/but_paths_should_be_ordered_by_length/aaaa"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_folders/but_paths_should_be_ordered_by_length/mmm"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_folders/but_paths_should_be_ordered_by_length/mmmmm"),
            ]
        );
    }
    #[test]
    fn but_paths_should_be_ordered_by_depth_of_folders() {
        let mut paths = vec![
            path_to_test_folder!("abcdefg"),
            path_to_test_folder!("nopqrst"),
            path_to_test_folder!("a/bcdefg"),
            path_to_test_folder!("no/pqrst"),
            path_to_test_folder!("u/v/w/x/y"),
            path_to_test_folder!("uv/wx/y"),
        ];
        paths.sort();
        assert_eq!(
            paths,
            vec![
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_folders/but_paths_should_be_ordered_by_depth_of_folders/abcdefg"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_folders/but_paths_should_be_ordered_by_depth_of_folders/nopqrst"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_folders/but_paths_should_be_ordered_by_depth_of_folders/a/bcdefg"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_folders/but_paths_should_be_ordered_by_depth_of_folders/no/pqrst"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_folders/but_paths_should_be_ordered_by_depth_of_folders/uv/wx/y"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_folders/but_paths_should_be_ordered_by_depth_of_folders/u/v/w/x/y"),
            ]
        );
    }
}

#[cfg(test)]
mod test_very_specific_ordering_files_and_dictories {
    use iocore::Path;
    use iocore_test::{path_to_test_file, path_to_test_folder};
    #[test]
    fn test_paths_should_be_ordered_alphabetically() {
        let mut paths = vec![
            path_to_test_file!("zzzzz"),
            path_to_test_file!("mmmmm/mmmmm"),
            path_to_test_folder!("mmmmm"),
            path_to_test_folder!("nnnnn"),
            path_to_test_file!("nnnnn/nnnn"),
            path_to_test_file!("aaaaa"),
        ];
        paths.sort();
        assert_eq!(
            paths,
            vec![
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/test_paths_should_be_ordered_alphabetically/aaaaa"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/test_paths_should_be_ordered_alphabetically/mmmmm"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/test_paths_should_be_ordered_alphabetically/nnnnn"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/test_paths_should_be_ordered_alphabetically/zzzzz"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/test_paths_should_be_ordered_alphabetically/mmmmm/mmmmm"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/test_paths_should_be_ordered_alphabetically/nnnnn/nnnn"),
            ]
        );
    }
    #[test]
    fn but_paths_should_be_ordered_by_length() {
        let mut paths = vec![
            path_to_test_file!("mmmmm"),
            path_to_test_folder!("mmm"),
            path_to_test_file!("aaa"),
            path_to_test_folder!("aaaa"),
        ];
        paths.sort();
        assert_eq!(
            paths,
            vec![
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/but_paths_should_be_ordered_by_length/aaa"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/but_paths_should_be_ordered_by_length/aaaa"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/but_paths_should_be_ordered_by_length/mmm"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/but_paths_should_be_ordered_by_length/mmmmm"),
            ]
        );
    }
    #[test]
    fn but_paths_should_be_ordered_by_depth_of_folders() {
        let mut paths = vec![
            path_to_test_file!("abcdefg"),
            path_to_test_file!("nopqrst"),
            path_to_test_file!("a/bcdefg"),
            path_to_test_folder!("a"),
            path_to_test_file!("no/pqrst"),
            path_to_test_folder!("u"),
            path_to_test_folder!("u/v"),
            path_to_test_folder!("u/v/w"),
            path_to_test_folder!("u/v/w/x"),
            path_to_test_file!("u/v/w/x/y"),
            path_to_test_file!("uv/wx/y"),
        ];
        paths.sort();
        assert_eq!(
            paths,
            vec![
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/but_paths_should_be_ordered_by_depth_of_folders/a"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/but_paths_should_be_ordered_by_depth_of_folders/u"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/but_paths_should_be_ordered_by_depth_of_folders/abcdefg"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/but_paths_should_be_ordered_by_depth_of_folders/nopqrst"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/but_paths_should_be_ordered_by_depth_of_folders/u/v"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/but_paths_should_be_ordered_by_depth_of_folders/a/bcdefg"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/but_paths_should_be_ordered_by_depth_of_folders/no/pqrst"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/but_paths_should_be_ordered_by_depth_of_folders/u/v/w"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/but_paths_should_be_ordered_by_depth_of_folders/u/v/w/x"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/but_paths_should_be_ordered_by_depth_of_folders/uv/wx/y"),
                Path::raw("tests/test_path_cmp/test_very_specific_ordering_files_and_dictories/but_paths_should_be_ordered_by_depth_of_folders/u/v/w/x/y"),
            ]
        );
    }
}
