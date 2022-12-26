use whistle_ast::Grammar;
use whistle_ast::Type;

mod checker;
mod expr;
mod program;
mod stmt;

pub use checker::*;
pub use expr::*;
pub use program::*;
pub use stmt::*;

pub fn check_all(checker: &mut Checker, grammar: &mut Grammar) {
  checker.scope.enter_scope();
  for program in grammar {
    check_program(checker, program);
  }
  checker.scope.exit_scope();
  for constraint in checker.constraints.clone() {
    checker.unify(constraint)
  }
  for (i, substitution) in checker.substitutions.clone().iter().enumerate() {
    checker.substitutions[i] = Checker::coerce(checker.substitute(substitution.clone()));
    if Type::Error == checker.substitutions[i] {
      println!("Could not infer type!")
    }
  }
}
