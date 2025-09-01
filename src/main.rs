mod helpers;
use helpers::{convert_defined_fields, convert_idl_type_names, convert_instruction_account_item};

use anchor_idl::IdlTypeDefTy;
use anchor_idl::{Idl, IdlTypeDefGeneric};
use anyhow::{Context, Result};
use clap::Parser;
use heck::{ToLowerCamelCase, ToUpperCamelCase};

#[derive(Parser)]
struct Cli {
    /// The path to the json file to convert.
    path: std::path::PathBuf,

    /// The directory to save the output file. Defaults to dir of json file.
    #[clap(short, long)]
    outdir: Option<std::path::PathBuf>,

    /// Name of the IDL type and exported const to generate. Defaults to the self-defined IDL name in metadata.
    #[clap(short = 'n', long)]
    idl_type_name: Option<String>,

    /// Additional metadata info added to the generated file.
    description: Option<String>,

    // Addtional metadata info on url to repository to be added to the generated file.
    repository: Option<String>,
}

pub fn idl_ts(idl: &mut Idl, idl_type_name: Option<String>) -> Result<String, std::io::Error> {
    // --- Accounts
    for acc in idl.accounts.iter_mut() {
        acc.name = acc.name.to_lower_camel_case();
    }
    // --- Events
    for event in idl.events.iter_mut() {
        event.name = event.name.to_lower_camel_case();
    }
    // --- Errors
    for error in idl.errors.iter_mut() {
        error.name = error.name.to_lower_camel_case();
    }
    // --- Types
    for type_def in idl.types.iter_mut() {
        type_def.name = type_def.name.to_lower_camel_case();
        match &mut type_def.ty {
            IdlTypeDefTy::Struct { fields } => {
                if let Some(fields) = fields {
                    convert_defined_fields(fields);
                }
            }
            IdlTypeDefTy::Enum { variants } => {
                for variant in variants.iter_mut() {
                    variant.name = variant.name.to_lower_camel_case();
                    if let Some(fields) = &mut variant.fields {
                        convert_defined_fields(fields);
                    }
                }
            }
            IdlTypeDefTy::Type { alias } => {
                convert_idl_type_names(alias);
            }
        }
        for generic in type_def.generics.iter_mut() {
            match generic {
                IdlTypeDefGeneric::Type { name } => {
                    *name = name.to_lower_camel_case();
                }
                IdlTypeDefGeneric::Const { name, .. } => {
                    *name = name.to_lower_camel_case();
                }
            }
        }
    }
    // --- Constants
    for constant in idl.constants.iter_mut() {
        constant.name = constant.name.to_lower_camel_case();
    }
    // --- Instructions
    for instruction in idl.instructions.iter_mut() {
        instruction.name = instruction.name.to_lower_camel_case();
        for arg in instruction.args.iter_mut() {
            arg.name = arg.name.to_lower_camel_case();
            convert_idl_type_names(&mut arg.ty);
        }
        for account in instruction.accounts.iter_mut() {
            convert_instruction_account_item(account);
        }
    }

    let idl_json = serde_json::to_string_pretty(&idl)?;
    let idl_name = idl_type_name.unwrap_or_else(|| idl.metadata.name.to_upper_camel_case());
    let type_name = format!("{idl_name}");
    let type_export = format!("export type {type_name} = {idl_json};");

    Ok(format!("{type_export}\n"))
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let bytes = std::fs::read(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    let mut idl: Idl = serde_json::from_slice(&bytes).expect("Invalid IDL format.");

    if let Some(description) = args.description {
        idl.metadata.description = Some(description);
    }
    if let Some(repository) = args.repository {
        idl.metadata.repository = Some(repository);
    }

    let ts_idl = idl_ts(&mut idl, args.idl_type_name)?;

    let mut default_path = args.path;
    default_path.pop();

    let out = args.outdir.unwrap_or_else(|| default_path);

    let ts_out = out.join(&idl.metadata.name).with_extension("ts");

    std::fs::write(&ts_out, ts_idl)?;
    println!("file created at: {}", ts_out.display());
    Ok(())
}
