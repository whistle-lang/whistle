use crate::chunk::{Chunk, OpCode};

#[derive(Debug)]
pub struct Engine {
  chunk: Chunk,
  ip: *const OpCode,
}

pub enum InterpretResult {
  Ok(),
  CompileError(),
  RuntimeError(),
}

impl Engine {
  pub fn new() -> Self {
    let chunk = Chunk::new();
    let ip = chunk.as_prt();
    Self { chunk, ip }
  }

  pub fn get_chunk(&mut self) -> &mut Chunk {
    &mut self.chunk
  }

  pub fn interpret(&mut self) -> InterpretResult {
    InterpretResult::Ok()
  }
}
