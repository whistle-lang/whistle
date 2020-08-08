#[derive(Debug, Clone, PartialEq)]
pub struct FloatLit {
  pub value: f64,
}

impl FloatLit {
    pub fn from(float: &str) -> FloatLit {
      let value = float.parse().unwrap();
      
      FloatLit {
        value
      }
    }
}

