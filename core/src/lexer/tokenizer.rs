#[derive(Debug, Clone)]
pub struct Tokenizer {
  pub source: Vec<char>,
  pub index: usize,
}

impl Tokenizer {
  pub fn new(source: &str) -> Self {
    Self {
      source: source.chars().collect(),
      index: 0,
    }
  }

  pub fn within_index(&self, i: usize) -> bool {
    if i < self.source.len() {
      return true;
    }

    false
  }

  pub fn within_offset(&self, offset: usize) -> bool {
    self.within_index(self.index + offset)
  }

  pub fn within(&self) -> bool {
    self.within_index(self.index)
  }

  pub fn peek_index(&self, i: usize) -> Option<char> {
    if self.within_index(i) {
      return Some(self.source[i]);
    }

    None
  }

  pub fn peek_offset(&self, offset: usize) -> Option<char> {
    self.peek_index(self.index + offset)
  }

  pub fn peek(&self) -> Option<char> {
    self.peek_index(self.index)
  }

  pub fn peek_range(&self, range: usize) -> Option<Vec<char>> {
    if self.within_offset(range - 1) {
      let mut out = Vec::new();

      for offset in 0..range {
        if let Some(ch) = self.peek_offset(offset) {
          out.push(ch);
        }
      }

      Some(out)
    } else {
      None
    }
  }

  pub fn is_char(&self, ch: char) -> bool {
    let curr = self.peek();

    if let Some(curr) = curr {
      if ch == curr {
        return true;
      }
    }

    false
  }

  pub fn is_str(&self, str: &str) -> bool {
    let range = self.peek_range(str.len()).unwrap_or(vec![]);
    let chars: Vec<char> = str.chars().collect();
    let chars = chars.as_slice();

    if range != chars {
      return false;
    }

    true
  }

  pub fn eat_char(&mut self, ch: char) -> Option<char> {
    if self.is_char(ch) {
      self.index += 1;
      Some(ch)
    } else {
      None
    }
  }

  pub fn eat_str(&mut self, str: &str) -> Option<String> {
    if self.is_str(str) {
      self.index += str.len();
      Some(String::from(str))
    } else {
      None
    }
  }

  pub fn step(&mut self) -> Option<char> {
    let character = self.peek();

    if self.within_offset(1) {
      self.index += 1;
    }

    character
  }

  pub fn steps(&mut self, n: usize) -> Option<Vec<char>> {
    if self.within_offset(n) {
      let mut out = Vec::new();

      for _ in 0..n {
        if let Some(ch) = self.step() {
          out.push(ch);
        }
      }

      Some(out)
    } else {
      None
    }
  }

  pub fn read_while<C>(&mut self, cond: C) -> Option<String>
  where
    C: Fn(char) -> bool,
  {
    let mut out = String::new();

    loop {
      if let Some(character) = self.peek() {
        if cond(character) {
          if let Some(ch) = self.step() {
            out.push(ch);
          }
        } else {
          break;
        }
      }
    }

    if out.is_empty() {
      None
    } else {
      Some(out)
    }
  }
}
