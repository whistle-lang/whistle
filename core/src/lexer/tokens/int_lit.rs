#[derive(Debug, Clone, PartialEq)]
pub struct IntLit {
  pub value: usize,
}

impl IntLit {
  pub fn from_binary(bin: &str) -> IntLit {
    let value = usize::from_str_radix(&*bin.chars().skip(2).collect::<String>(), 2).unwrap();

    IntLit { value }
  }

  pub fn from_octal(oct: &str) -> IntLit {
    let value = usize::from_str_radix(&*oct.chars().skip(2).collect::<String>(), 8).unwrap();

    IntLit { value }
  }

  pub fn from_hex(hex: &str) -> IntLit {
    let value = usize::from_str_radix(&*hex.chars().skip(2).collect::<String>(), 16).unwrap();

    IntLit { value }
  }

  pub fn from_decimal(dec: &str) -> IntLit {
    let value = usize::from_str_radix(dec, 10).unwrap();

    IntLit { value }
  }
}
