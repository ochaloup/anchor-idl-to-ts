use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[clap(
  author,
  version,
  about = "Convert Anchor JSON IDL files to Anchor TypeScript types"
)]
pub struct Cli {
  /// The path to the JSON file to convert
  pub path: std::path::PathBuf,

  /// File to write the output to (defaults is a file named based on the IDL type name defined in the IDL)
  #[clap(short, long)]
  pub out: Option<std::path::PathBuf>,

  /// Name of the IDL type and exported const to generate
  /// (defaults to the self-defined IDL name or name in metadata)
  #[clap(short = 'n', long)]
  pub idl_type_name: Option<String>,

  /// Additional metadata description to be added to the generated file (new format only)
  #[clap(short = 'd', long)]
  pub description: Option<String>,

  /// Additional metadata repository URL to be added to the generated file (new format only)
  #[clap(short = 'r', long)]
  pub repository: Option<String>,

  /// Force specific IDL version instead of auto-detecting
  #[clap(short = 'f', long, value_enum)]
  pub idl_version: Option<IdlVersion>,

  /// Verbose output
  #[clap(short, long)]
  pub verbose: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum IdlVersion {
  /// Old format for anchor 0.29.0 and prior versions, it has 'name' field at root level
  Old,
  /// New format for anchor 0.30.0 and later versions, it has 'metadata.name' field
  New,
}
