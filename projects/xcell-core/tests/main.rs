use env_logger::builder;
use log::LevelFilter;

use xcell_core::{ProjectConfig, PROJECT_CONFIG};

// use sled_typed::{Database, DiskMap};

mod test_buffer;

#[test]
fn ready() {
    println!("it works!")
}

pub fn logger() {
    let _ = builder().filter(Some("globset"), LevelFilter::Off).filter_level(LevelFilter::Trace).try_init();
}

#[test]
fn project_config_default() {
    println!("{:#?}", toml::from_str::<ProjectConfig>(PROJECT_CONFIG).unwrap())
}
// #[test]
// fn test_files() {
//     let path = PathBuf::from("sqlite");
//     let db = Database::open(&path).unwrap();
//     println!("{:#?}", db);
//     let map: DiskMap<String, String> = db.document("").unwrap();
//     let key = "key".to_string();
//     let value = "value".to_string();
//     map.insert(key, value);
//     println!("{:?}", map.get("key".to_string()));
//     // file_db.test().await.unwrap()
// }
