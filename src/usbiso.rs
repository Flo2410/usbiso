use anyhow::{Error, Ok};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha256::try_digest;
use std::{
  fs::OpenOptions,
  io::{BufReader, Read},
  path::Path,
};

use crate::{database::DbEntry, downloader::download_file, utils::open_json_file};

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
  root_folder: Box<Path>,

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
      root_folder: path.to_path_buf().into_boxed_path(),
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

  fn save_lock(&self) -> anyhow::Result<()> {
    serde_json::to_writer_pretty(
      OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&self.lock_file_path)?,
      &self.lock,
    )?;
    Ok(())
  }

  pub async fn add(&mut self, db_entry: &DbEntry) -> anyhow::Result<()> {
    let manifest_entry = ManifestEntry {
      name: db_entry.name.clone(),
      version: String::from("TODO"),
    };

    self.manifest.isos.push(manifest_entry);

    // TODO: berfore downlowding, check if file exists. If yes: compare checksum.
    let hash_download_res = download_file(&Client::new(), &db_entry.hash_url, &self.root_folder).await?;
    let iso_download_res = download_file(&Client::new(), &db_entry.iso_url, &self.root_folder).await?;

    let iso_hash_calc = try_digest(iso_download_res.path).unwrap();

    let mut hash_file = OpenOptions::new().read(true).open(hash_download_res.path)?;

    let mut real_hash = String::new();
    hash_file.read_to_string(&mut real_hash)?;

    real_hash = real_hash.split_ascii_whitespace().nth(0).unwrap().to_string();

    assert_eq!(real_hash, iso_hash_calc);

    let lock_entry = LockEntry {
      name: db_entry.name.clone(),
      version: String::from("TODO"),
      url: iso_download_res.url.to_string(),
      hash: iso_hash_calc,
    };

    self.lock.isos.push(lock_entry);
    self.save_lock()?;

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

    // check if in lock
    let lock_index_option = self.lock.isos.iter().position(|x| x.name == name);
    if lock_index_option.is_none() {
      // println!();
      return Err(Error::msg(format!(
        "The ISO \"{}\" does not exist in the lockfile",
        name
      )));
    }

    let lock_index = lock_index_option.unwrap();

    self.lock.isos.swap_remove(lock_index);
    self.save_lock()?;

    Ok(())
  }
}
