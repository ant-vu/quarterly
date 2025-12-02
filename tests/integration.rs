use quarterly::count_lines_in_file;
use std::path::PathBuf;

#[test]
fn example_data_count() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("data").join("example.csv");
    let count = count_lines_in_file(path).expect("read example data");
    assert_eq!(count, 3, "expected 3 non-empty lines in example.csv");
}
