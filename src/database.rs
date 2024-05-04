use serde::{Deserialize, Serialize};

const DATABSE: &str = include_str!("../assets/database.json");

#[derive(Serialize, Deserialize)]
pub struct DbEntry {
  pub name: String,
  pub display_name: String,
  pub iso_url: String,
  pub hash_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Database {
  pub isos: Vec<DbEntry>,
}

impl Database {
  pub fn load() -> Self {
    serde_json::from_str(DATABSE).expect("Could not parse database")
  }
}
