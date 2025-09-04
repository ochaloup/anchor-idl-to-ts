mod cli;
mod converters;
mod helpers;

use crate::converters::{detect_idl_version, process_new_idl, process_old_idl};
use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, IdlVersion};

fn main() -> Result<()> {
  let args = Cli::parse();

  let bytes = std::fs::read(&args.path)
    .with_context(|| format!("could not read file `{}`", args.path.display()))?;

  let version = match args.idl_version {
    Some(version) => version,
    None => detect_idl_version(&bytes)?,
  };

  let (ts_output, idl_type_name) = match version {
    IdlVersion::Old => process_old_idl(&bytes, &args)?,
    IdlVersion::New => process_new_idl(&bytes, &args)?,
  };

  let mut default_path = args.path.clone();
  default_path.pop();
  let ts_out = args
    .out
    .unwrap_or(default_path.join(&idl_type_name).with_extension("ts"));

  std::fs::write(&ts_out, ts_output)?;
  println!("✓ File created at: {}", ts_out.display());

  if args.verbose {
    println!("  IDL version: {:?}", version);
    println!("  Output name: {}", idl_type_name);
  }

  Ok(())
}
