pub use whistle_common::Literal;
pub use whistle_common::Operator;
pub use whistle_common::Primitive;
pub use whistle_common::Tip;

use std::collections::HashMap;

/// https://whistle.js.org/docs/specification/grammar#identifiers
#[derive(Debug, Clone, PartialEq)]
pub enum IdentType {
  Ident(String),
  Union {
    lhs: Box<IdentType>,
    rhs: Box<IdentType>,
  },
  IdentType {
    ident: String,
    prim: Vec<IdentType>,
  },
  Struct(Vec<IdentTypedStrict>),
  Primitive(Primitive),
  Function {
    params: HashMap<String, Var>,
    ret_type: Box<IdentType>
  },
  Undefined,
  Error
}

#[derive(Debug, Clone, PartialEq)]
pub struct Var {
  pub mutable: bool,
  pub types: IdentType,
}

/// https://whistle.js.org/docs/specification/grammar#identifiers
#[derive(Debug, Clone, PartialEq)]
pub struct IdentFunc {
  pub ident: String,
  pub generic: Option<String>,
}

/// https://whistle.js.org/docs/specification/grammar#identifiers
#[derive(Debug, Clone, PartialEq)]
pub struct IdentTypedStrict {
  pub ident: String,
  pub type_ident: IdentType,
}

/// https://whistle.js.org/docs/specification/grammar#identifiers
#[derive(Debug, Clone, PartialEq)]
pub struct IdentTyped {
  pub ident: String,
  pub type_ident: Option<IdentType>,
}

/// https://whistle.js.org/docs/specification/grammar#identifiers
#[derive(Debug, Clone, PartialEq)]
pub struct IdentImport {
  pub ident: String,
  pub as_ident: Option<String>,
}

/// https://whistle.js.org/docs/specification/grammar#expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
  Unary(Unary),
  Binary {
    op: Operator,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
  },
  Cond {
    then_expr: Box<Expr>,
    cond: Box<Expr>,
    else_expr: Box<Expr>,
  },
}

/// https://whistle.js.org/docs/specification/grammar#expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Unary {
  Primary(Primary),
  UnaryOp { op: Operator, expr: Box<Unary> },
}

/// https://whistle.js.org/docs/specification/grammar#expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Primary {
  Literal(Literal),
  IdentVal { ident: String, prim: Vec<IdentVal> },
  Grouping(Box<Expr>),
}

/// https://whistle.js.org/docs/specification/grammar#expressions
#[derive(Debug, Clone, PartialEq)]
pub enum IdentVal {
  Selector(String),
  Arguments(Vec<Expr>),
  Index(Box<Expr>),
  Slice {
    start: usize,
    end: usize,
    step: usize,
  },
}

/// https://whistle.js.org/docs/specification/grammar#statements
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
  Block(Vec<Stmt>),
  Tip(Tip),
  Expr(Expr),
  Assign {
    rhs: Box<Expr>,
    op: Box<Stmt>,
    lhs: Box<Expr>,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgramStmt {
  Import {
    idents: Vec<IdentImport>,
    from: String,
  },
  FunDecl {
    export: bool,
    ident: String,
    params: Vec<IdentTyped>,
    ret_type: Option<IdentType>,
    stmt: Vec<Stmt>,
  },
  VarDecl {
    ident_typed: IdentTyped,
    val: Box<Expr>,
  },
  ValDecl {
    ident_typed: IdentTyped,
    val: Box<Expr>,
  },
  StructDecl {
    export: bool,
    ident: String,
    params: Vec<IdentTypedStrict>,
  },
  TypeDecl {
    export: bool,
    ident: String,
    types: IdentType,
  },
  Stmt(Stmt),
}

/// https://whistle.js.org/docs/specification/grammar#grammar
pub type Grammar = Vec<ProgramStmt>;
