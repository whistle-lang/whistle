use crate::Compiler;

use whistle_ast::Grammar;

mod expr;
mod program;
mod stmt;
mod types;

pub use expr::*;
pub use program::*;
pub use stmt::*;
pub use types::*;

pub fn compile_grammar(compiler: &mut Compiler, grammar: Grammar) -> Vec<u8> {
  compiler.scope.enter_scope();

  for program in grammar {
    compile_program(compiler, program);
  }

  compiler.scope.exit_scope();
  compiler.module.finish()
}
