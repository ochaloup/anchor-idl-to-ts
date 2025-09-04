use crate::cli::{Cli, IdlVersion};
use anyhow::{Context, Result};
use heck::{ToLowerCamelCase, ToUpperCamelCase};
use serde_json::Value;

/// Detect IDL version by checking for 'metadata' field in JSON
pub fn detect_idl_version(bytes: &[u8]) -> Result<IdlVersion> {
  let json: Value =
    serde_json::from_slice(bytes).context("Failed to parse JSON for version detection")?;

  // Check if metadata.name exists (new format) or just name (old format)
  if json.get("metadata").and_then(|m| m.get("name")).is_some() {
    Ok(IdlVersion::New)
  } else if json.get("name").is_some() {
    Ok(IdlVersion::Old)
  } else {
    anyhow::bail!("Could not determine IDL format - neither 'name' nor 'metadata.name' found")
  }
}

/// Process old format IDL (0.3.0)
/// Returns (TypeScript output, IDL type name)
pub fn process_old_idl(bytes: &[u8], args: &Cli) -> Result<(String, String)> {
  use anchor_idl_030::Idl;

  let mut idl: Idl =
    serde_json::from_slice(bytes).context("Failed to parse IDL in old format (0.3.0)")?;

  // Convert account names to camelCase
  for acc in idl.accounts.iter_mut() {
    acc.name = acc.name.to_lower_camel_case();
  }

  let idl_json = serde_json::to_string_pretty(&idl)?;
  let idl_name = args
    .idl_type_name
    .clone()
    .unwrap_or_else(|| idl.name.to_upper_camel_case());

  let type_name = idl_name.to_string();
  let const_name = "IDL";

  let output = format!(
            "export type {type_name} = {idl_json};\n\nexport const {const_name}: {type_name} = {idl_json};\n"
        );

  Ok((output, idl.name))
}

/// Process new format IDL (0.4.1)
/// Returns (TypeScript output, IDL type name)
pub fn process_new_idl(bytes: &[u8], args: &Cli) -> Result<(String, String)> {
  use anchor_idl_041::Idl;

  let mut idl: Idl =
    serde_json::from_slice(bytes).context("Failed to parse IDL in new format (0.4.1)")?;

  // Apply metadata updates if provided
  if let Some(description) = &args.description {
    idl.metadata.description = Some(description.clone());
  }
  if let Some(repository) = &args.repository {
    idl.metadata.repository = Some(repository.clone());
  }

  // Convert all names to camelCase
  convert_new_idl(&mut idl)?;

  let idl_json = serde_json::to_string_pretty(&idl)?;
  let idl_name = args
    .idl_type_name
    .clone()
    .unwrap_or_else(|| idl.metadata.name.to_upper_camel_case());
  let type_name = idl_name.to_string();

  let output = format!("export type {type_name} = {idl_json};\n");

  Ok((output, idl.metadata.name.clone()))
}

/// Convert all names in the new IDL format to camelCase
fn convert_new_idl(idl: &mut anchor_idl_041::Idl) -> Result<()> {
  use crate::helpers;
  use anchor_idl_041::{IdlTypeDefGeneric, IdlTypeDefTy};

  // Accounts
  for acc in idl.accounts.iter_mut() {
    acc.name = acc.name.to_lower_camel_case();
  }

  // Events
  for event in idl.events.iter_mut() {
    event.name = event.name.to_lower_camel_case();
  }

  // Errors
  for error in idl.errors.iter_mut() {
    error.name = error.name.to_lower_camel_case();
  }

  // Types
  for type_def in idl.types.iter_mut() {
    type_def.name = type_def.name.to_lower_camel_case();

    match &mut type_def.ty {
      IdlTypeDefTy::Struct { fields } => {
        if let Some(fields) = fields {
          helpers::convert_defined_fields(fields);
        }
      }
      IdlTypeDefTy::Enum { variants } => {
        for variant in variants.iter_mut() {
          variant.name = variant.name.to_lower_camel_case();
          if let Some(fields) = &mut variant.fields {
            helpers::convert_defined_fields(fields);
          }
        }
      }
      IdlTypeDefTy::Type { alias } => {
        helpers::convert_idl_type_names(alias);
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

  // Constants
  for constant in idl.constants.iter_mut() {
    constant.name = constant.name.to_lower_camel_case();
  }

  // Instructions
  for instruction in idl.instructions.iter_mut() {
    instruction.name = instruction.name.to_lower_camel_case();

    for arg in instruction.args.iter_mut() {
      arg.name = arg.name.to_lower_camel_case();
      helpers::convert_idl_type_names(&mut arg.ty);
    }

    for account in instruction.accounts.iter_mut() {
      helpers::convert_instruction_account_item(account);
    }
  }

  Ok(())
}
