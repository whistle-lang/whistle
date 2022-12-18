use whistle_ast::Grammar;
use whistle_ast::IdentType;
use whistle_ast::IdentTyped;
use whistle_ast::Literal;
use whistle_ast::Primitive;
use whistle_ast::Type;
use whistle_ast::Typed;

mod checker;
mod expr;
mod program;
mod stmt;

pub use checker::*;
pub use expr::*;
pub use program::*;
pub use stmt::*;

pub fn check_grammar(checker: &mut Checker, grammar: &mut Grammar) {
  checker.scope.enter_scope();
  for program in grammar {
    check_program(checker, program);
  }
  checker.scope.exit_scope();
  for (type1, type2) in checker.constraints.clone() {
    checker.unify(type1, type2)
  }
  for (i, substitution) in checker.substitutions.clone().iter().enumerate() {
    checker.substitutions[i] = Checker::coerce(checker.substitute(substitution.clone()));
    if Type::Error == checker.substitutions[i] {
      println!("Could not infer type!")
    }
  }
  for (i, ptr) in checker.literals.clone() {
    unsafe {
      let sub = checker.substitutions[i].clone();
      *ptr = match &*ptr {
        Literal::Int(val) => match sub {
          Type::Primitive(Primitive::Int) => Literal::Int(*val),
          Type::Primitive(Primitive::I32) => Literal::I32(*val),
          Type::Primitive(Primitive::I64) => Literal::I64(*val),
          Type::Primitive(Primitive::U32) => Literal::U32(*val),
          Type::Primitive(Primitive::U64) => Literal::U64(*val),
          Type::Primitive(Primitive::F32) => Literal::F32(*val as f64),
          Type::Primitive(Primitive::F64) => Literal::F64(*val as f64),
          _ => unreachable!(),
        },
        Literal::Float(val) => match sub {
          Type::Primitive(Primitive::Float) => Literal::Float(*val),
          Type::Primitive(Primitive::F32) => Literal::F32(*val),
          Type::Primitive(Primitive::F64) => Literal::F64(*val),
          _ => unreachable!(),
        },
        _ => unreachable!(),
      };
    }
  }

  for (i, ptr) in checker.idents.clone() {
    unsafe { *ptr = assign_type(checker.substitutions[i].clone()) }
  }
}

pub fn assign_type(types: Type) -> IdentType {
  match types.clone() {
    Type::Ident(ident) => IdentType::Ident { ident, range: None },
    Type::Generic(var) => IdentType::Generic { var, range: None },
    Type::Var(..) => panic!("Could not infer type!"),
    Type::IdentType { ident, prim } => IdentType::IdentType {
      ident,
      prim: assign_vec_type(prim),
      range: None,
    },
    Type::Struct(ident) => {
      let ident = assign_vec_typed(ident);
      IdentType::Struct { ident, range: None }
    }
    Type::Primitive(prim) => IdentType::Primitive { prim, range: None },
    Type::Function { params, ret_type } => IdentType::Function {
      params: assign_vec_typed(params),
      ret_type: Box::new(assign_type(*ret_type)),
      range: None,
    },
    Type::Array(ident) => {
      let ident = Box::new(assign_type(*ident));
      IdentType::Array { ident, range: None }
    }
    Type::Default => IdentType::Default,
    Type::Error => IdentType::Error,
  }
}

pub fn assign_vec_type(types: Vec<Type>) -> Vec<IdentType> {
  types.iter().map(|x| assign_type(x.clone())).collect()
}

pub fn assign_vec_typed(types: Vec<Typed>) -> Vec<IdentTyped> {
  types
    .iter()
    .map(|x| IdentTyped {
      ident: x.ident.clone(),
      type_ident: assign_type(x.type_ident.clone()),
      range: None,
    })
    .collect()
}
