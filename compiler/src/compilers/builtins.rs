use crate::ident_type_to_val_type;
use crate::Compiler;
use crate::CompilerErrorKind;
use crate::Symbol;
use wasm_encoder::EntityType;
use wasm_encoder::ValType;
use whistle_ast::IdentBuiltin;
use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
use whistle_ast::Primitive;

pub fn setup_builtin(compiler: &mut Compiler, namespace: &str, fn_name: &str, types: IdentType) {
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
    let encoded_ret_type: Vec<ValType> = if *ret_type == IdentType::Primitive(Primitive::None) { vec![] } else { vec![ident_type_to_val_type(*ret_type)] };
    compiler.module.types.function(param_types, encoded_ret_type);
  }
}

pub fn compile_builtins_io(compiler: &mut Compiler, idents: Vec<IdentBuiltin>) {
  for builtin in idents {
    let types = match builtin.ident.as_str() {
      "println" => IdentType::Function {
        params: vec![IdentTyped {
          ident: String::from("value"),
          type_ident: IdentType::Primitive(Primitive::Str),
        }],
        ret_type: Box::new(IdentType::Primitive(Primitive::None)),
      },
      "printInt" => IdentType::Function {
        params: vec![IdentTyped {
          ident: String::from("value"),
          type_ident: IdentType::Primitive(Primitive::I32),
        }],
        ret_type: Box::new(IdentType::Primitive(Primitive::None)),
      },
      _ => {
        compiler.throw(CompilerErrorKind::Unimplemented, 0);
        IdentType::Error
      }
    };
    setup_builtin(compiler, "io", builtin.ident.as_str(), types);
  }
}

pub fn compile_builtins_core(compiler: &mut Compiler, idents: Vec<IdentBuiltin>) {
  for builtin in idents {
    let types = match builtin.ident.as_str() {
      "fd_close" => IdentType::Function {
        params: vec![IdentTyped {
          ident: String::from("fd"),
          type_ident: IdentType::Primitive(Primitive::I32),
        }],
        ret_type: Box::new(IdentType::Primitive(Primitive::I32)),
      },
      "fd_datasync" => IdentType::Function {
        params: vec![IdentTyped {
          ident: String::from("fd"),
          type_ident: IdentType::Primitive(Primitive::I32),
        }],
        ret_type: Box::new(IdentType::Primitive(Primitive::I32)),
      },
      "fd_fdstat_get" => IdentType::Function {
        params: vec![
          IdentTyped {
            ident: String::from("fd"),
            type_ident: IdentType::Primitive(Primitive::I32),
          },
          IdentTyped {
            ident: String::from("offset"),
            type_ident: IdentType::Primitive(Primitive::I32),
          },
        ],
        ret_type: Box::new(IdentType::Primitive(Primitive::I32)),
      },
      "fd_fdstat_set_flags" => IdentType::Function {
        params: vec![
          IdentTyped {
            ident: String::from("fd"),
            type_ident: IdentType::Primitive(Primitive::I32),
          },
          IdentTyped {
            ident: String::from("flags"),
            type_ident: IdentType::Primitive(Primitive::I32),
          },
        ],
        ret_type: Box::new(IdentType::Primitive(Primitive::I32)),
      },
      "fd_sync" => IdentType::Function {
        params: vec![IdentTyped {
          ident: String::from("fd"),
          type_ident: IdentType::Primitive(Primitive::I32),
        }],
        ret_type: Box::new(IdentType::Primitive(Primitive::I32)),
      },
      "proc_exit" => IdentType::Function {
        params: vec![IdentTyped {
          ident: String::from("rval"),
          type_ident: IdentType::Primitive(Primitive::I32),
        }],
        ret_type: Box::new(IdentType::Primitive(Primitive::None)),
      },
      "proc_raise" => IdentType::Function {
        params: vec![IdentTyped {
          ident: String::from("sig"),
          type_ident: IdentType::Primitive(Primitive::I32),
        }],
        ret_type: Box::new(IdentType::Primitive(Primitive::I32)),
      },
      "sched_yield" => IdentType::Function {
        params: vec![],
        ret_type: Box::new(IdentType::Primitive(Primitive::I32)),
      },
      _ => {
        compiler.throw(CompilerErrorKind::Unimplemented, 0);
        IdentType::Error
      }
    };
    setup_builtin(compiler, "wasi_unstable", builtin.ident.as_str(), types);
  }
}
