use crate::lexer::{Operator, Tip};

// https://whistle.js.org/docs/specification/grammar#identifiers

#[derive(Debug, Clone, PartialEq)]
pub struct IdentTyped {
  pub ident: String,
  pub type_ident: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentImport {
  pub ident: String,
  pub as_ident: Option<String>,
}

// https://whistle.js.org/docs/specification/grammar#literals

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
  Float(f64),
  Int(usize),
  Str(String),
  Char(char),
  Bool(bool),
  None,
}

// https://whistle.js.org/docs/specification/grammar#exprs

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
  Unary(UnaryExpr),
  Binary {
    op: Operator,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
  },
  Cond {
    cond: Box<Expr>,
    then_expr: Box<Expr>,
    else_expr: Box<Expr>,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryExpr {
  Primary(PrimaryExpr),
  UnaryOp { op: Operator, expr: Box<UnaryExpr> },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimaryExpr {
  Operand(Operand),
  Selector {
    prim: Box<PrimaryExpr>,
    ident: String,
  },
  Arguments {
    prim: Box<PrimaryExpr>,
    args: Vec<Expr>,
  },
  Index {
    prim: Box<PrimaryExpr>,
    idx: usize,
  },
  Slice {
    prim: Box<PrimaryExpr>,
    start: usize,
    end: usize,
    step: usize,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
  Literal(Literal),
  Ident(String),
  Grouping(Box<Expr>),
}

// https://whistle.js.org/docs/specification/grammar#statements

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
  If {
    cond: Box<Expr>,
    then_stmt: Box<Stmt>,
    else_stmt: Option<Box<Stmt>>,
  },
  While {
    cond: Option<Box<Expr>>,
    do_stmt: Box<Stmt>,
  },
  Continue,
  Break,
  Return(Option<Box<Expr>>),
  VarDecl {
    ident_typed: IdentTyped,
    val: Box<Expr>,
  },
  ValDecl {
    ident_typed: IdentTyped,
    val: Box<Expr>,
  },
  FunDecl {
    ident: String,
    params: Option<Vec<IdentTyped>>,
    ret_type: String,
    stmt: Box<Stmt>,
  },
  Block(Vec<Stmt>),
  Import {
    idents: Vec<IdentImport>,
    from: String,
  },
  Tip(Tip),
  Expr(Expr),
}

// https://whistle.js.org/docs/specification/grammar#grammar

pub type Grammar = Vec<Stmt>;
