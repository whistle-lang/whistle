use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
  Float(f32),
  Double(f64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValuePool {
  values: Vec<Value>,
}

impl ValuePool {
  pub fn new() -> Self {
    Self { values: vec![] }
  }
  pub fn write(&mut self, value: Value) {
    self.values.push(value);
  }
  pub fn len(&self) -> usize {
    self.values.len()
  }
}
