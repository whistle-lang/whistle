use whistle_common::Tip;

use crate::{Compiler, Function};

pub fn compile_tip_wasm_bytes(
  _compiler: &mut Compiler,
  function: &mut Function,
  tip: Tip,
  _span: whistle_common::Span,
) {
  let raw_data = tip
    .value
    .split(",")
    .map(|s| s.trim())
    .collect::<Vec<&str>>();
  let data = raw_data
    .iter()
    .map(|s| s.parse::<u8>().unwrap())
    .collect::<Vec<u8>>();
  function.raw(data);
}

pub fn compile_tip_wast(compiler: &mut Compiler, tip: Tip, _span: whistle_common::Span) {
  let lexer = wast::lexer::Lexer::new(tip.value.as_str());
  let buf = wast::parser::ParseBuffer::new_with_lexer(lexer).unwrap();
  let ast = wast::parser::parse::<wast::Wat>(&buf)
    .unwrap()
    .encode()
    .unwrap();
  compiler.module.code.raw(&ast);
}
