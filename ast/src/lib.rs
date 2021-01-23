pub use whistle_lexer::Operator;
pub use whistle_lexer::Tip;

/// https://whistle.js.org/docs/specification/grammar#identifiers
#[derive(Debug, Clone, PartialEq)]
pub struct IdentTyped {
  pub ident: String,
  pub type_ident: String,
}

/// https://whistle.js.org/docs/specification/grammar#identifiers
#[derive(Debug, Clone, PartialEq)]
pub struct IdentImport {
  pub ident: String,
  pub as_ident: Option<String>,
}

/// https://whistle.js.org/docs/specification/grammar#literals
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
  Float(f64),
  Int(usize),
  Str(String),
  Char(char),
  Bool(bool),
  None,
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
  Operand(Operand),
  Selector {
    prim: Box<Primary>,
    ident: String,
  },
  Arguments {
    prim: Box<Primary>,
    args: Vec<Expr>,
  },
  Index {
    prim: Box<Primary>,
    idx: usize,
  },
  Slice {
    prim: Box<Primary>,
    start: usize,
    end: usize,
    step: usize,
  },
}

/// https://whistle.js.org/docs/specification/grammar#expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
  Literal(Literal),
  Ident(String),
  Grouping(Box<Expr>),
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgramStmt {
  Import {
    idents: Vec<IdentImport>,
    from: String,
  },
  FunDecl {
    ident: String,
    params: Vec<Vec<IdentTyped>>,
    ret_type: String,
    stmt: Box<Stmt>,
  },
  VarDecl {
    ident_typed: IdentTyped,
    val: Box<Expr>,
  },
  ValDecl {
    ident_typed: IdentTyped,
    val: Box<Expr>,
  },
}

/// https://whistle.js.org/docs/specification/grammar#grammar
pub type Grammar = Vec<ProgramStmt>;
