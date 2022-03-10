use wasm_encoder::MemArg;
use wasm_encoder::MemoryType;

// command line flag for target arch
pub enum MemoryAlignment {
  Bit32,
  Bit64,
}

pub struct Memory {
  pub align: u32,
  pub stack: u64,
  pub _heap: u64,
}

impl Memory {
  pub fn new() -> Self {
    Self {
      align: 8,
      stack: 0,
      _heap: 0,
    }
  }

  pub fn alloc(&self) -> MemoryType {
    MemoryType {
      minimum: 1,
      maximum: None,
      memory64: false,
    }
  }

  pub fn step_stack(&mut self, length: u64) {
    self.stack += length
  }

  pub fn index_stack(&self) -> MemArg {
    MemArg {
      align: self.align,
      memory_index: 0,
      offset: self.stack,
    }
  }
}
