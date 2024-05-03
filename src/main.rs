mod args;

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs::File, io::BufReader, path::Path};

use args::{ActionType, CliArgs};

const DATABSE: &str = include_str!("../assets/database.json");

#[derive(Serialize, Deserialize)]
struct DbEntry {
  name: String,
  iso_url: String,
  hash_url: String,
}

#[derive(Serialize, Deserialize)]
struct Database {
  isos: Vec<DbEntry>,
}

#[tokio::main]
async fn main() {
  let args = CliArgs::parse();

  let file_name = args.path.unwrap_or(String::from("./usbiso.json"));
  let file_path = Path::new(&file_name);

  if !file_path.exists() {
    let file = File::create(file_path).unwrap_or_else(|_| panic!("Could not create file: {}", file_path.display()));
    serde_json::to_writer(&file, &json!({})).expect("Could not write to file");
  }

  let file = File::open(file_path).unwrap_or_else(|_| panic!("Could not open file: {}", file_path.display()));

  let reader = BufReader::new(file);

  let value: Value = serde_json::from_reader(reader).expect("Could not parse json");
  let db: Database = serde_json::from_str(DATABSE).expect("Could not parse database");

  match args.action {
    ActionType::List(list_args) => {
      // List all available ISOs from database
      if list_args.available {
        println!("ISOs available in the database:");
        db.isos.iter().for_each(|iso| println!("{}", iso.name));
        return;
      }

      todo!("List...");
    }
    ActionType::Add(_) => {}
    ActionType::Remove(action_args) => todo!("Remove... {:?}", action_args.iso_name),
  };
}
