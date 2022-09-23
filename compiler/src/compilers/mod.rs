use crate::Compiler;
use crate::CompilerError;
use crate::Symbol;

use wasm_encoder::DataSegment;
use wasm_encoder::DataSegmentMode;
use wasm_encoder::EntityType;
use wasm_encoder::ExportKind;
use wasm_encoder::ConstExpr;
use whistle_ast::Grammar;
use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
use whistle_ast::Primitive;

mod expr;
mod program;
mod stmt;
mod types;

pub use expr::*;
pub use program::*;
pub use stmt::*;
pub use types::*;

pub fn compile_grammar(
  compiler: &mut Compiler,
  grammar: Grammar,
) -> Result<Vec<u8>, Vec<CompilerError>> {
  compiler.module.memories.memory(compiler.memory.alloc());
  compiler.scope.enter_scope();
  setup_builtins(compiler);
  for program in grammar {
    compile_program(compiler, program);
  }

  compiler.scope.exit_scope();
  compiler.module.data.segment(DataSegment {
    data: compiler.memory.buf.clone(),
    mode: DataSegmentMode::Active {
      memory_index: 0,
      offset: &ConstExpr::i32_const(0),

    },
  });
  compiler.module.exports.export("memory", ExportKind::Memory, 0);
  if compiler.errors.is_empty() {
    Ok(compiler.module.finish())
  } else {
    Err(compiler.errors.clone())
  }
}

pub fn setup_builtins(compiler: &mut Compiler) {
  setup_builtin(
    compiler,
    "sys",
    "printInt",
    IdentType::Function {
      params: vec![IdentTyped {
        ident: String::from("value"),
        type_ident: IdentType::Primitive(Primitive::I32),
      }],
      ret_type: Box::new(IdentType::Primitive(Primitive::None)),
    },
  );

  setup_builtin(
    compiler,
    "sys",
    "printString",
    IdentType::Function {
      params: vec![IdentTyped {
        ident: String::from("value"),
        type_ident: IdentType::Primitive(Primitive::Str),
      }],
      ret_type: Box::new(IdentType::Primitive(Primitive::None)),
    },
  );
}

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
    let ret_type = ident_type_to_val_type(*ret_type);
    compiler.module.types.function(param_types, vec![ret_type]);
  }
}
