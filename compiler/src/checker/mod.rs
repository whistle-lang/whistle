use whistle_ast::Grammar;
use whistle_ast::IdentType;
use whistle_ast::Literal;
use whistle_ast::Primitive;

mod checker;
mod expr;
mod program;
mod stmt;

pub use checker::*;
pub use expr::*;
pub use program::*;
pub use stmt::*;

pub fn check_grammar(checker: &mut Checker, grammar: Grammar) {
  checker.scope.enter_scope();
  for program in grammar {
    check_program(checker, program);
  }
  checker.scope.exit_scope();

  for (type1, type2) in checker.constraints.clone() {
    checker.unify(type1, type2)
  }
  for i in 0..checker.substitutions.len() {
    checker.substitutions[i] = checker.substitute(checker.substitutions[i].clone())
  }
  for (i, ptr) in checker.literals.clone() {
    unsafe {
      let sub = checker.substitutions[i].clone();
      *ptr = match &*ptr {
        Literal::Int(val) => match sub {
          IdentType::Primitive(Primitive::Int) => Literal::Int(*val),
          IdentType::Primitive(Primitive::I32) => Literal::I32(*val),
          IdentType::Primitive(Primitive::I64) => Literal::I64(*val),
          IdentType::Primitive(Primitive::U32) => Literal::U32(*val),
          IdentType::Primitive(Primitive::U64) => Literal::U64(*val),
          IdentType::Primitive(Primitive::F32) => Literal::F32(*val as f64),
          IdentType::Primitive(Primitive::F64) => Literal::F64(*val as f64),
          _ => unreachable!(),
        },
        Literal::Float(val) => match sub {
          IdentType::Primitive(Primitive::Float) => Literal::Float(*val),
          IdentType::Primitive(Primitive::F32) => Literal::F32(*val),
          IdentType::Primitive(Primitive::F64) => Literal::F64(*val),
          _ => unreachable!(),
        },
        _ => unreachable!(),
      };
    }
  }

  for (i, ptr) in checker.idents.clone() {
    unsafe { (*ptr).type_ident = checker.substitutions[i].clone() }
  }
}
