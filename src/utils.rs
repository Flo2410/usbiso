use serde_json::json;
use std::{
  fs::{File, OpenOptions},
  path::Path,
};

pub fn open_json_file(path: &Path) -> File {
  if !path.exists() {
    let file = File::create(path).unwrap_or_else(|_| panic!("Could not create file: {}", path.display()));
    serde_json::to_writer_pretty(&file, &json!({"isos":[]})).expect("Could not write to file");
  }

  OpenOptions::new()
    .read(true)
    .open(path)
    .unwrap_or_else(|_| panic!("Could not open file: {}", path.display()))
}
