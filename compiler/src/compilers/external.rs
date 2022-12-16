use crate::ident_type_to_val_type;
use crate::Compiler;
use crate::Symbol;
use wasm_encoder::EntityType;
use wasm_encoder::ValType;
use whistle_ast::IdentType;
use whistle_ast::Primitive;

pub fn setup_extern(compiler: &mut Compiler, namespace: &str, fn_name: &str, types: IdentType) {
  let res = compiler.scope.set_function_sym(
    fn_name,
    Symbol {
      global: true,
      mutable: false,
      types: types.clone(),
    },
  );
  let idx = match res {
    Ok(idx) => idx,
    Err(err) => {
      compiler.throw(err, 0);
      0
    }
  };
  compiler
    .module
    .imports
    .import(namespace, fn_name, EntityType::Function(idx));
  if let IdentType::Function { params, ret_type } = types {
    let mut param_types = Vec::new();
    for param in params {
      param_types.push(ident_type_to_val_type(param.type_ident));
    }
    let encoded_ret_type: Vec<ValType> = if *ret_type == IdentType::Primitive(Primitive::None) {
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
