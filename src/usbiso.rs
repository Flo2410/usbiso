use anyhow::{Error, Ok};
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::BufReader, path::Path};

use crate::{database::DbEntry, utils::open_json_file};

#[derive(Serialize, Deserialize)]
pub struct ManifestEntry {
  pub name: String,
  pub version: String,
}

#[derive(Serialize, Deserialize)]
pub struct Manifest {
  pub isos: Vec<ManifestEntry>,
}

#[derive(Serialize, Deserialize)]
pub struct LockEntry {
  pub name: String,
  pub version: String,
  pub url: String,
  pub hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct LockFile {
  pub isos: Vec<LockEntry>,
}

pub struct UsbIso {
  manifest_file_path: Box<Path>,
  pub manifest: Manifest,

  lock_file_path: Box<Path>,
  pub lock: LockFile,
}

impl UsbIso {
  pub fn load(path: &Path) -> Self {
    // Load manifest
    let manifest_file_path = path.join("usbiso.json").into_boxed_path();
    let manifest_file = open_json_file(&manifest_file_path);
    let manifest = serde_json::from_reader(BufReader::new(&manifest_file)).expect("Could not parse manifest");

    // Load lock
    let lock_file_path = path.join("usbiso.lock").into_boxed_path();
    let lock_file = open_json_file(&lock_file_path);
    let lock = serde_json::from_reader(BufReader::new(&lock_file)).expect("Could not parse lock file");

    Self {
      manifest_file_path,
      manifest,
      lock_file_path,
      lock,
    }
  }

  fn save_manifest(&self) -> anyhow::Result<()> {
    serde_json::to_writer_pretty(
      OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&self.manifest_file_path)?,
      &self.manifest,
    )?;
    Ok(())
  }

  pub fn add(&mut self, db_entry: &DbEntry) -> anyhow::Result<()> {
    let manifest_entry = ManifestEntry {
      name: db_entry.name.clone(),
      version: String::from("TODO"),
    };

    self.manifest.isos.push(manifest_entry);
    self.save_manifest()?;
    Ok(())
  }

  pub fn remove(&mut self, name: String) -> anyhow::Result<()> {
    // check if in manifest
    let manifest_index_option = self.manifest.isos.iter().position(|x| x.name == name);
    if manifest_index_option.is_none() {
      // println!();
      return Err(Error::msg(format!(
        "The ISO \"{}\" does not exist in the manifest",
        name
      )));
    }

    let manifest_index = manifest_index_option.unwrap();

    self.manifest.isos.swap_remove(manifest_index);
    self.save_manifest()?;
    Ok(())
  }
}
