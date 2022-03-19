use whistle_ast::Grammar;

mod expr;
mod program;
mod stmt;
mod checker;

pub use expr::*;
pub use program::*;
pub use stmt::*;
pub use checker::*;

pub fn check_grammar(checker: &mut Checker, grammar: Grammar) {
  checker.scope.enter_scope();
  for program in grammar {
    check_program(checker, program);
  }

  checker.scope.exit_scope();
}