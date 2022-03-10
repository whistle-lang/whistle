use crate::Compiler;
use crate::CompilerError;

use whistle_ast::Grammar;

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
  compiler.module.memories.memory(
    compiler.memory.alloc()
  );
  compiler.scope.enter_scope();

  for program in grammar {
    compile_program(compiler, program);
  }

  compiler.scope.exit_scope();

  if compiler.errors.is_empty() {
    Ok(compiler.module.finish())
  } else {
    Err(compiler.errors.clone())
  }
}
