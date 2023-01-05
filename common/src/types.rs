use crate::Primitive;

#[derive(Debug, Clone, PartialEq)]
pub struct TypedIdent {
  pub ident: String,
  pub type_ident: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  Ident(String),
  Generic(String),
  Var(usize),
  IdentType {
    ident: String,
    prim: Vec<Type>,
  },
  Struct(Vec<TypedIdent>),
  Primitive(Primitive),
  Function {
    params: Vec<TypedIdent>,
    ret_type: Box<Type>,
  },
  Array(Box<Type>),
  Default,
  Error,
}
