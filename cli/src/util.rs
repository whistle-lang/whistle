use whistle_ast::Grammar;
// use whistle_compiler::compiler::compile_all;
// use whistle_compiler::compiler::Compiler;
// use whistle_compiler::compilers::compile_grammar;
use whistle_common::TokenItem;
use whistle_lexer::*;
use whistle_parser::*;

pub fn lex(text: &str) -> Vec<TokenItem> {
  let lexer = Lexer::new(text);
  let mut toks = Vec::new();

  for tok in lexer {
    match tok {
      Ok(tok) => toks.push(tok.clone()),
      Err(err) => {
        println!("{:?}", err);
        std::process::exit(1);
      }
    }
  }

  toks
}

pub fn parse(text: &str) -> Grammar {
  let tokens = lex(text);
  let parser = &mut Parser::new(tokens);

  match parse_all(parser) {
    Ok(val) => {
      print!("{:?}", val);
      val
    }
    Err(err) => {
      println!("{:?}", err);
      std::process::exit(1);
    }
  }
}

// pub fn compile(text: &str) -> Vec<u8> {
// let grammar = parse(text);
// let compiler = &mut Compiler::new();
// compile_grammar(compiler, grammar);
// compile_all(compiler)
// }
