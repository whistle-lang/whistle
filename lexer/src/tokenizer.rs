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

  pub fn peek_offset(&self, offset: isize) -> Option<char> {
    self.peek_index((self.index as isize + offset) as usize)
  }

  pub fn peek(&self) -> Option<char> {
    self.peek_index(self.index)
  }

  pub fn peek_range(&self, range: usize) -> Option<Vec<char>> {
    if self.within_offset(range - 1) {
      let mut out = Vec::new();

      for offset in 0..range {
        if let Some(ch) = self.peek_offset(offset as isize) {
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
    let range = self.peek_range(str.len()).unwrap_or_default();
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
    if self.within() {
      self.index += 1;
      self.peek_offset(-1)
    } else {
      None
    }
  }

  pub fn read_while<C>(&mut self, cond: C) -> Option<String>
  where
    C: Fn(char) -> bool,
  {
    let mut out = String::new();

    while let Some(ch) = self.peek() {
      if cond(ch) {
        out.push(self.step().unwrap());
      } else {
        break;
      }
    }

    if out.is_empty() {
      None
    } else {
      Some(out)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::Tokenizer;

  #[test]
  fn within_index() {
    let tokenizer = Tokenizer::new("a");

    assert!(tokenizer.within_index(0));
    assert!(!tokenizer.within_index(1));
  }

  #[test]
  fn within_offset() {
    let mut tokenizer = Tokenizer::new("a");

    assert!(tokenizer.within_offset(0));
    assert!(!tokenizer.within_offset(1));
    tokenizer.index += 1;
    assert!(!tokenizer.within_offset(0));
    assert!(!tokenizer.within_offset(1));
  }

  #[test]
  fn within() {
    let mut tokenizer = Tokenizer::new("a");

    assert!(tokenizer.within());
    tokenizer.index += 1;
    assert!(!tokenizer.within());
  }

  #[test]
  fn peek_index() {
    let tokenizer = Tokenizer::new("abc");

    assert_eq!(tokenizer.peek_index(0), Some('a'));
    assert_eq!(tokenizer.peek_index(1), Some('b'));
    assert_eq!(tokenizer.peek_index(2), Some('c'));
    assert_eq!(tokenizer.peek_index(3), None);
  }

  #[test]
  fn peek_offset() {
    let mut tokenizer = Tokenizer::new("abc");

    assert_eq!(tokenizer.peek_offset(0), Some('a'));
    assert_eq!(tokenizer.peek_offset(1), Some('b'));
    assert_eq!(tokenizer.peek_offset(2), Some('c'));
    assert_eq!(tokenizer.peek_offset(3), None);
    tokenizer.index += 1;
    assert_eq!(tokenizer.peek_offset(0), Some('b'));
    assert_eq!(tokenizer.peek_offset(1), Some('c'));
    assert_eq!(tokenizer.peek_offset(2), None);
    assert_eq!(tokenizer.peek_offset(3), None);
  }

  #[test]
  fn peek() {
    let mut tokenizer = Tokenizer::new("abc");

    assert_eq!(tokenizer.peek(), Some('a'));
    tokenizer.index += 1;
    assert_eq!(tokenizer.peek(), Some('b'));
    tokenizer.index += 1;
    assert_eq!(tokenizer.peek(), Some('c'));
    tokenizer.index += 1;
    assert_eq!(tokenizer.peek(), None);
  }

  #[test]
  fn is_char() {
    let mut tokenizer = Tokenizer::new("a");

    assert!(tokenizer.is_char('a'));
    tokenizer.index += 1;
    assert!(!tokenizer.is_char('a'));
  }

  #[test]
  fn is_str() {
    let mut tokenizer = Tokenizer::new("abc");

    assert!(tokenizer.is_str("abc"));
    assert!(!tokenizer.is_str("abcd"));
    tokenizer.index += 1;
    assert!(tokenizer.is_str("bc"));
    assert!(!tokenizer.is_str("abc"));
  }

  #[test]
  fn eat_char() {
    let mut tokenizer = Tokenizer::new("ab");

    assert_eq!(tokenizer.eat_char('b'), None);
    assert_eq!(tokenizer.eat_char('a'), Some('a'));
    assert_eq!(tokenizer.eat_char('a'), None);
    assert_eq!(tokenizer.eat_char('b'), Some('b'));
  }

  #[test]
  fn eat_str() {
    let mut tokenizer = Tokenizer::new("abcd");

    assert_eq!(tokenizer.eat_str("cd"), None);
    assert_eq!(tokenizer.eat_str("ab"), Some(String::from("ab")));
    assert_eq!(tokenizer.eat_str("ef"), None);
    assert_eq!(tokenizer.eat_str("cd"), Some(String::from("cd")));
  }

  #[test]
  fn step() {
    let mut tokenizer = Tokenizer::new("ab");

    assert_eq!(tokenizer.index, 0);
    assert_eq!(tokenizer.step(), Some('a'));
    assert_eq!(tokenizer.index, 1);
    assert_eq!(tokenizer.step(), Some('b'));
    assert_eq!(tokenizer.index, 2);
    assert_eq!(tokenizer.step(), None);
    assert_eq!(tokenizer.index, 2);
  }

  #[test]
  fn read_while() {
    let mut tokenizer = Tokenizer::new("aaaabbbb");

    assert_eq!(
      tokenizer.read_while(|ch| ch == 'a'),
      Some(String::from("aaaa"))
    );
    assert_eq!(tokenizer.read_while(|ch| ch == 'a'), None);
    assert_eq!(
      tokenizer.read_while(|ch| ch == 'b'),
      Some(String::from("bbbb"))
    );
    assert_eq!(tokenizer.read_while(|ch| ch == 'b'), None);
  }
}
