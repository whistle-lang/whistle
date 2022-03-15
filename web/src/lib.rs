//#[macro_use]
use wasm_bindgen::prelude::wasm_bindgen;
use whistle_lexer::*;
use whistle_parser::*;
mod utils;
use utils::*;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn lex(text: String) -> String {
  let lexer = Lexer::new(&text);
  let mut toks = Vec::new();
  for tok in lexer {
    match tok {
      Ok(tok) => {
        println!("{:?}", tok);
        toks.push(tok.clone())
      }
      Err(err) => {
        println!("{:?}", err);
        break;
      }
    }
  }

  format!("{:?}", toks)
}

#[wasm_bindgen]
pub fn parse(text: String) -> String {
  let tokens = lexthing(&text, false);
  let parser = &mut Parser::new(tokens);

  match parse_all(parser) {
    Ok(val) => {
      format!("{:#?}", val)
    }
    Err(err) => {
      format!("{:?}", err)
    }
  }
}

#[wasm_bindgen]
pub fn compile(text: String) -> String {
  let bytes = compilething(&text);
  format!("{:#?}", &bytes[..])
}
