pub use whistle_common::Literal;
pub use whistle_common::Operator;
pub use whistle_common::Primitive;
pub use whistle_common::Tip;

#[derive(Debug, Clone, PartialEq)]
pub enum IdentType {
  Ident(String),
  Generic(String),
  Var(usize),
  IdentType {
    ident: String,
    prim: Vec<IdentType>,
  },
  Struct(Vec<IdentTyped>),
  Primitive(Primitive),
  Function {
    params: Vec<IdentTyped>,
    ret_type: Box<IdentType>,
  },
  Array(Box<IdentType>),
  Default,
  Error,
}

/// https://whistle.js.org/docs/specification/grammar#identifiers
#[derive(Debug, Clone, PartialEq)]
pub struct IdentFunc {
  pub ident: String,
  pub generic: Vec<String>,
}

/// https://whistle.js.org/docs/specification/grammar#identifiers
#[derive(Debug, Clone, PartialEq)]
pub struct IdentTyped {
  pub ident: String,
  pub type_ident: IdentType,
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
    cond: Box<Expr>,
    then_expr: Box<Expr>,
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
  IdentVal {
    ident: String,
    prim: Vec<IdentVal>,
  },
  Grouping(Box<Expr>),
  Array {
    exprs: Vec<Expr>,
    type_ident: IdentType,
  },
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
    cond: Expr,
    then_stmt: Vec<Stmt>,
    else_stmt: Option<Vec<Stmt>>,
  },
  While {
    cond: Expr,
    do_stmt: Vec<Stmt>,
  },
  Continue,
  Break,
  Return(Option<Expr>),
  VarDecl {
    ident_typed: IdentTyped,
    val: Expr,
  },
  ValDecl {
    ident_typed: IdentTyped,
    val: Expr,
  },
  Block(Vec<Stmt>),
  Tip(Tip),
  Expr(Expr),
  Assign {
    ident: String,
    op: Operator,
    rhs: Expr,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgramStmt {
  Import {
    idents: Vec<IdentImport>,
    from: String,
    imp_type: String,
  },
  FnDecl {
    export: bool,
    inline: bool,
    ident: String,
    params: Vec<IdentTyped>,
    ret_type: IdentType,
    stmt: Vec<Stmt>,
  },
  VarDecl {
    ident_typed: IdentTyped,
    val: Expr,
  },
  ValDecl {
    ident_typed: IdentTyped,
    val: Expr,
  },
  StructDecl {
    export: bool,
    ident: String,
    params: Vec<IdentTyped>,
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
