use iocore::Path;
use iocore_test::folder_path;
#[test]
fn test_rust_core_library_panic() {
    let fixtures_path = folder_path!("fixtures");
    let mut paths = vec![
        fixtures_path.join("tests/fixtures/tree/b/bc/bcbb/4fb78e02ff67d8012fbb88d40d3a31f3"),
        fixtures_path.join("tests/fixtures/tree/a/a3/a370/a37028/db529185677e9dad0f03c79cacfe7344"),
        fixtures_path.join("tests/fixtures/tree/5/50/50ff/0f1552df1c1d05e81da68f63cdc19c41"),
        fixtures_path.join("tests/fixtures/tree/a/a4/a4eb/a4eb41/e85f8ea54f5628479572c13bb3f5db47"),
        fixtures_path.join("tests/fixtures/tree/b/b5/b560/4546ab772208a4c50ee5bb59d6049657"),
        fixtures_path.join("tests/fixtures/tree/a/a4/a4eb/65e5a532ef56e8d0e71078c703b140bf"),
    ];
    paths.sort();
    assert_eq!(
        paths,
        vec![
            Path::raw("tests/fixtures/tests/fixtures/tree/5/50/50ff/0f1552df1c1d05e81da68f63cdc19c41"),
            Path::raw("tests/fixtures/tests/fixtures/tree/a/a4/a4eb/65e5a532ef56e8d0e71078c703b140bf"),
            Path::raw("tests/fixtures/tests/fixtures/tree/b/b5/b560/4546ab772208a4c50ee5bb59d6049657"),
            Path::raw("tests/fixtures/tests/fixtures/tree/b/bc/bcbb/4fb78e02ff67d8012fbb88d40d3a31f3"),
            Path::raw("tests/fixtures/tests/fixtures/tree/a/a3/a370/a37028/db529185677e9dad0f03c79cacfe7344"),
            Path::raw("tests/fixtures/tests/fixtures/tree/a/a4/a4eb/a4eb41/e85f8ea54f5628479572c13bb3f5db47"),
        ]
    );
}
