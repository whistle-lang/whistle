mod expr;
pub use expr::*;
mod ident;
pub use ident::*;
mod literal;
pub use literal::*;
mod stmt;
pub use stmt::*;

pub fn parse_grammar(parser: &mut Parser) {
  let mut stmts: Vec<Stmt> = Vec::new();
  while parser.within() {
    if let Some(result) = parse_stmt(parser) {
      stmts.push(result)
    } else {
      println!("ALERT: ParseError at {}", parser.index);
      break;
    }
  }
  println!(
    "
  Result: 
  {:?}",
    stmts
  );
}
