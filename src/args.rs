use clap::{Args, Parser, Subcommand};

/// A simple CLI for managing your ISOs.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
  /// list, add or remove ISOs
  #[clap(subcommand)]
  pub action: ActionType,

  /// The path to your usbiso folder.
  #[arg(short, long)]
  pub path: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum ActionType {
  /// List all ISOs you have in your folder.
  List(ListArgs),

  /// Add an ISO
  Add(ActionArgs),

  /// Remove an ISO
  Remove(ActionArgs),
}

#[derive(Debug, Args)]
pub struct ActionArgs {
  /// The name of the ISO.
  pub iso_name: String,
}

#[derive(Debug, Args)]
pub struct ListArgs {
  /// List all availabel ISOs in the database
  #[arg(short, long)]
  pub available: bool,
}
