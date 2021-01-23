use crate::compiler::Compiler;
use crate::compiler::Function;
use crate::compilers::compile_stmt;
use crate::types::Type;

use whistle_ast::Grammar;
use whistle_ast::IdentTyped;
use whistle_ast::ProgramStmt;
use whistle_ast::Stmt;

pub fn compile_grammar(compiler: &mut Compiler, grammar: Grammar) {
  for program in grammar {
    compile_program(compiler, program)
  }
}

pub fn compile_program(compiler: &mut Compiler, program: ProgramStmt) {
  match program {
    ProgramStmt::FunDecl {
      ident,
      params,
      ret_type,
      stmt,
    } => compile_function(compiler, &ident, params, &ret_type, *stmt),
    ProgramStmt::ValDecl { .. } => panic!("Global vals are not yet supported"),
    ProgramStmt::VarDecl { .. } => panic!("Global vars are not yet supported"),
    ProgramStmt::Import { .. } => panic!("Imports are not yet supported"),
  }
}

pub fn compile_function(
  compiler: &mut Compiler,
  ident: &str,
  params: Vec<Vec<IdentTyped>>,
  ret_type: &str,
  stmt: Stmt,
) {
  compiler.func = Function::new();

  let params: Vec<IdentTyped> = params.into_iter().flatten().collect();
  for param in params {
    compiler.set_param(&param.ident, Type::from(&param.type_ident));
  }

  compiler.func.index = compiler.funcs.len();
  compiler.func.result_types.push(Type::from(&ret_type));
  compile_stmt(compiler, stmt);
  compiler.set_func(ident, compiler.func.clone());
}
