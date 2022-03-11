use crate::Compiler;
use crate::CompilerError;

use wasm_encoder::Instruction;
use wasm_encoder::DataSegmentMode;
use wasm_encoder::DataSegment;
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
  compiler.module.data.segment(DataSegment {
    data: compiler.memory.buf.clone(),
    mode: DataSegmentMode::Active {
      memory_index: 0,
      offset: &Instruction::I32Const(0)
    }
  });
  if compiler.errors.is_empty() {
    Ok(compiler.module.finish())
  } else {
    Err(compiler.errors.clone())
  }
}
