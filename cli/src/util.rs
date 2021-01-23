use whistle_ast::Grammar;
use whistle_compiler::compiler::compile_all;
use whistle_compiler::compiler::Compiler;
use whistle_compiler::compilers::compile_grammar;
use whistle_lexer::*;
use whistle_parser::*;

pub fn lex(text: &str) -> Vec<TokenItem> {
  let lexer = Lexer::new(text);
  let mut toks = Vec::new();

  for tok in lexer {
    if let Ok(tok) = tok {
      toks.push(tok.clone());
    } else {
      break;
    }
  }

  toks
}

pub fn parse(text: &str) -> Grammar {
  let lexer = Lexer::new(text);
  let parser = &mut Parser::from(lexer);
  parse_grammar(parser)
}

pub fn compile(text: &str) -> Vec<u8> {
  let grammar = parse(text);
  let compiler = &mut Compiler::new();
  compile_grammar(compiler, grammar);
  compile_all(compiler)
}
