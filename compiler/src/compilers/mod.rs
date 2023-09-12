use crate::Compiler;

use wasm_encoder::ConstExpr;
use wasm_encoder::DataSegment;
use wasm_encoder::DataSegmentMode;
use wasm_encoder::ExportKind;
use whistle_ast::Grammar;

mod expr;
mod external;
mod program;
mod stmt;
mod types;
mod tip;

pub use expr::*;
pub use external::*;
pub use program::*;
pub use stmt::*;
pub use types::*;
pub use tip::*;

pub fn compile_all(compiler: &mut Compiler, grammar: Grammar) -> Vec<u8> {
  compiler.module.memories.memory(compiler.memory.alloc());
  for program in grammar {
    compile_program(compiler, program);
  }
  compiler.module.data.segment(DataSegment {
    data: compiler.memory.buf.clone(),
    mode: DataSegmentMode::Active {
      memory_index: 0,
      offset: &ConstExpr::i32_const(0),
    },
  });
  compiler
    .module
    .exports
    .export("memory", ExportKind::Memory, 0);
  compiler.module.finish()
}
