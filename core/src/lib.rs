#[macro_use]
extern crate enum_kind;

pub mod lexer;

pub fn version() -> &'static str {
  env!("CARGO_PKG_VERSION")
}
