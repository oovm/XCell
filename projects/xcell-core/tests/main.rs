use log::LevelFilter;
use std::path::PathBuf;

use sled_typed::{Database, DiskMap};

#[test]
fn ready() {
    println!("it works!")
}

pub fn logger() {
    let _ = env_logger::builder().filter_level(LevelFilter::Off).is_test(true).try_init();
}

mod test_buffer;

#[test]
fn test_files() {
    let path = PathBuf::from("sqlite");
    let db = Database::open(&path).unwrap();
    println!("{:#?}", db);
    let map: DiskMap<String, String> = db.document("").unwrap();
    let key = "key".to_string();
    let value = "value".to_string();
    map.insert(key, value);
    println!("{:?}", map.get("key".to_string()));
    // file_db.test().await.unwrap()
}
