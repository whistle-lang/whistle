/// A basic implementation of run-length encoding
/// source: https://en.wikipedia.org/wiki/Run-length_encoding
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Line(usize, usize);

#[derive(Debug, Serialize, Deserialize)]
pub struct LineBuffer {
  line: Vec<Line>,
}

impl LineBuffer {
  pub fn new() -> Self {
    Self { line: vec![] }
  }
  pub fn push(&mut self, line: usize) {
    let len = self.line.len();
    if len == 0 {
      self.line.push(Line(1, line));
    } else {
      let prev = self.line.get_mut(len - 1).unwrap();
      if prev.1 == line {
        prev.0 += 1;
      } else {
        self.line.push(Line(1, line));
      }
    }
  }

  #[allow(dead_code)]
  pub fn get(&self, index: usize) -> Option<usize> {
    let mut sum: usize = 0;
    for curr in &self.line {
      sum += curr.0;
      if index <= sum {
        return Some(curr.1);
      }
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use super::LineBuffer;

  #[test]
  fn no_lines() {
    let lines = LineBuffer::new();
    assert_eq!(lines.get(1), None);
  }

  #[test]
  fn only_one_line() {
    let mut lines = LineBuffer::new();
    for _ in 0..32 {
      lines.push(1);
    }
    assert_eq!(lines.get(0), Some(1));
    assert_eq!(lines.get(32), Some(1));
    assert_eq!(lines.get(33), None);
  }

  #[test]
  fn two_lines() {
    let mut lines = LineBuffer::new();
    for _ in 0..32 {
      lines.push(1);
    }
    for _ in 0..32 {
      lines.push(2);
    }
    assert_eq!(lines.get(0), Some(1));
    assert_eq!(lines.get(32), Some(1));
    assert_eq!(lines.get(33), Some(2));
    assert_eq!(lines.get(64), Some(2));
    assert_eq!(lines.get(65), None);
  }
}
