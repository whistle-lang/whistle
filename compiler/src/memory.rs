use byteorder::{ByteOrder, LittleEndian};
use wasm_encoder::MemArg;
use wasm_encoder::MemoryType;

use whistle_common::Literal;
use whistle_common::Primitive;

// command line flag for target arch
pub enum MemoryAlignment {
  Bit32,
  Bit64,
}

pub struct Memory {
  pub align: u32,
  pub stack: u64,
  pub _heap: u64,
  pub buf: Vec<u8>,
}

impl Memory {
  pub fn new() -> Self {
    Self {
      align: 8,
      stack: 0,
      _heap: 0,
      buf: Vec::new(),
    }
  }

  pub fn alloc(&self) -> MemoryType {
    MemoryType {
      minimum: 1,
      maximum: None,
      memory64: false,
      shared: false
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

  pub fn write_prim(&mut self, prim: Primitive, lit: Literal) {
    match prim {
      Primitive::F32 => {
        if let Literal::Float(n) = lit {
          LittleEndian::write_u32(&mut self.buf, (n as f32).to_bits())
        }
      }
      Primitive::F64 => {
        if let Literal::Float(n) = lit {
          LittleEndian::write_u64(&mut self.buf, n.to_bits())
        }
      }
      Primitive::U32 => {
        if let Literal::Int(n) = lit {
          LittleEndian::write_u32(&mut self.buf, n as u32)
        }
      }
      Primitive::U64 => {
        if let Literal::Int(n) = lit {
          LittleEndian::write_u64(&mut self.buf, n as u64)
        }
      }
      Primitive::Char => {
        if let Literal::Char(c) = lit {
          LittleEndian::write_u32(&mut self.buf, c as u32)
        }
      }
      Primitive::Bool => {
        if let Literal::Bool(b) = lit {
          LittleEndian::write_u32(&mut self.buf, b as u32)
        }
      }
      _ => unimplemented!(),
    }
  }
}

impl Default for Memory {
  fn default() -> Self {
    Self::new()
  }
}
