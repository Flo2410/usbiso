mod args;
mod database;
mod downloader;
mod usbiso;
mod utils;

use anyhow::Ok;
use clap::Parser;
use std::path::Path;

use args::{ActionType, CliArgs};
use database::Database;
use usbiso::UsbIso;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let args = CliArgs::parse();

  let root_foler_name = args.path.unwrap_or(String::from("./")); // Get usbiso folder form arg or use current folder
  let root_folder = Path::new(&root_foler_name);

  let mut usbiso = UsbIso::load(root_folder);
  let db = Database::load();

  match args.action {
    ActionType::List(list_args) => {
      // List all available ISOs from database
      if list_args.available {
        println!("ISOs available in the database:");
        db.isos
          .iter()
          .for_each(|iso| println!("{} : {}", iso.display_name, iso.name));
        return Ok(());
      }

      if usbiso.manifest.isos.is_empty() {
        println!("There are no ISOs in the manifest");
        return Ok(());
      }

      println!("ISOs in the manifest:");
      usbiso
        .manifest
        .isos
        .iter()
        .for_each(|iso| println!("{} : {}", iso.name, iso.version));
    }

    ActionType::Add(action_args) => {
      let name = action_args.iso_name;

      // check if already in manifest
      if usbiso.manifest.isos.iter().any(|x| x.name == name) {
        println!("The ISO \"{}\" is already in the manifest", name);
        return Ok(());
      }

      // Check if available in db
      let db_index_option = db.isos.iter().position(|x| x.name == name);
      if db_index_option.is_none() {
        println!("The ISO \"{}\" does not exist in the database", name);
        return Ok(());
      }

      let db_entry = db
        .isos
        .get(db_index_option.unwrap())
        .expect("Could not get entry from db");

      println!("Adding {}", name);

      usbiso.add(db_entry).await?;
    }
    ActionType::Remove(action_args) => {
      let name = action_args.iso_name;

      println!("Removing {}", name);

      usbiso.remove(name)?;
    }
  };

  Ok(())
}
