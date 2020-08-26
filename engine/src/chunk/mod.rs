mod line;

use bincode::{deserialize, serialize, Result};
use serde::{Deserialize, Serialize};

use crate::value::{Value, ValuePool};

use line::LineBuffer;

#[derive(Debug, Serialize, Deserialize)]
pub enum OpCode {
  Constant(usize),
  Return,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chunk {
  code: Vec<OpCode>,
  line: LineBuffer,
  constants: ValuePool,
}

impl Chunk {
  pub fn new() -> Self {
    Self {
      code: vec![],
      line: LineBuffer::new(),
      constants: ValuePool::new(),
    }
  }

  pub fn deserialize(bytes: &[u8]) -> Result<Self> {
    deserialize(bytes)
  }

  pub fn serialize(&self) -> Result<Vec<u8>> {
    serialize(self)
  }

  pub fn as_prt(&self) -> *const OpCode {
    self.code.as_ptr()
  }
  pub fn write(&mut self, op: OpCode, line: usize) {
    self.code.push(op);
    self.line.push(line);
  }

  pub fn add_constant(&mut self, value: Value) -> usize {
    self.constants.write(value);
    self.constants.len() - 1
  }
}
