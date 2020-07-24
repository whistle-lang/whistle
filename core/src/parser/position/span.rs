use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct SpanData {
  pub lo: BytePos,
  pub hi: BytePos,
}

pub trait Pos {
  fn from_usize(n: usize) -> Self;
  fn to_usize(&self) -> usize;
  fn from_u32(n: u32) -> Self;
  fn to_u32(&self) -> u32;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug, Serialize, Deserialize)]
pub struct BytePos(pub u32);

impl Pos for BytePos {
  #[inline(always)]
  fn from_usize(n: usize) -> BytePos {
    BytePos(n as u32)
  }

  #[inline(always)]
  fn to_usize(&self) -> usize {
    self.0 as usize
  }

  #[inline(always)]
  fn from_u32(n: u32) -> BytePos {
    BytePos(n)
  }

  #[inline(always)]
  fn to_u32(&self) -> u32 {
    self.0
  }
}

impl Add for BytePos {
  type Output = BytePos;

  #[inline(always)]
  fn add(self, rhs: BytePos) -> BytePos {
    BytePos((self.to_usize() + rhs.to_usize()) as u32)
  }
}

impl Sub for BytePos {
  type Output = BytePos;

  #[inline(always)]
  fn sub(self, rhs: BytePos) -> BytePos {
    BytePos((self.to_usize() - rhs.to_usize()) as u32)
  }
}
