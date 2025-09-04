use anchor_idl_041::{IdlDefinedFields, IdlGenericArg, IdlInstructionAccountItem, IdlType};
use heck::ToLowerCamelCase;

pub(crate) fn convert_instruction_account_item(item: &mut IdlInstructionAccountItem) {
  match item {
    IdlInstructionAccountItem::Composite(composite) => {
      composite.name = composite.name.to_lower_camel_case();
      for nested_account in composite.accounts.iter_mut() {
        convert_instruction_account_item(nested_account);
      }
    }
    IdlInstructionAccountItem::Single(single) => {
      single.name = single.name.to_lower_camel_case();
    }
  }
}

// Helper function to convert field names in IdlDefinedFields
pub(crate) fn convert_defined_fields(fields: &mut IdlDefinedFields) {
  match fields {
    IdlDefinedFields::Named(named_fields) => {
      for field in named_fields.iter_mut() {
        field.name = field.name.to_lower_camel_case();
        convert_idl_type_names(&mut field.ty);
      }
    }
    IdlDefinedFields::Tuple(tuple_types) => {
      for ty in tuple_types.iter_mut() {
        convert_idl_type_names(ty);
      }
    }
  }
}

// Helper function to convert names within IdlType recursively
pub(crate) fn convert_idl_type_names(idl_type: &mut IdlType) {
  match idl_type {
    IdlType::Option(inner) => {
      convert_idl_type_names(inner);
    }
    IdlType::Vec(inner) => {
      convert_idl_type_names(inner);
    }
    IdlType::Array(inner, _) => {
      convert_idl_type_names(inner);
    }
    IdlType::Defined { name, generics } => {
      *name = name.to_lower_camel_case();

      for generic in generics.iter_mut() {
        match generic {
          IdlGenericArg::Type { ty } => {
            convert_idl_type_names(ty);
          }
          IdlGenericArg::Const { .. } => {}
        }
      }
    }
    IdlType::Generic(name) => {
      *name = name.to_lower_camel_case();
    }
    _ => {}
  }
}
