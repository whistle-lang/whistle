use crate::ident_type_to_val_type;
use crate::Compiler;
use wasm_encoder::EntityType;
use wasm_encoder::ValType;
use whistle_ast::Type;

pub fn setup_extern(compiler: &mut Compiler, namespace: &str, fn_name: &str, types: Type) {
  let sym = compiler.get_sym(fn_name).unwrap();
  compiler
    .module
    .imports
    .import(namespace, fn_name, EntityType::Function(sym.0));
  if let Type::Function { params, ret_type } = types {
    let mut param_types = Vec::new();
    for param in params {
      param_types.push(ident_type_to_val_type(param.type_ident));
    }
    let encoded_ret_type: Vec<ValType> = if let Type::Primitive(..) = *ret_type {
      vec![]
    } else {
      vec![ident_type_to_val_type(*ret_type)]
    };
    compiler
      .module
      .types
      .function(param_types, encoded_ret_type);
  }
}
