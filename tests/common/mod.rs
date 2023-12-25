use std::fs::File;

pub fn get_fixture(path: &str) -> serde_json::Value {
    let file = File::open(path)
        .expect("file should open read only");
    serde_json::from_reader(file).expect("file should be JSON")
}
