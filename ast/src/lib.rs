pub use whistle_common::Literal;
pub use whistle_common::Operator;
pub use whistle_common::Primitive;
pub use whistle_common::Range;
pub use whistle_common::Tip;

#[derive(Debug, Clone, PartialEq)]
pub enum IdentType {
  Ident {
    ident: String,
    range: Range,
  },
  Generic {
    var: String,
    range: Range,
  },
  Var {
    var: usize,
    range: Range,
  },
  IdentType {
    ident: String,
    prim: Vec<IdentType>,
    range: Range,
  },
  Struct {
    ident: Vec<IdentTyped>,
    range: Range,
  },
  Primitive {
    prim: Primitive,
    range: Range,
  },
  Function {
    params: Vec<IdentTyped>,
    ret_type: Box<IdentType>,
    range: Range,
  },
  Array {
    ident: Box<IdentType>,
    range: Range,
  },
  Default,
  Error,
}

/// https://whistle.js.org/docs/specification/grammar#identifiers
#[derive(Debug, Clone, PartialEq)]
pub struct IdentFunction {
  pub ident: String,
  pub generic: Vec<String>,
  pub range: Range,
}

/// https://whistle.js.org/docs/specification/grammar#identifiers
#[derive(Debug, Clone, PartialEq)]
pub struct IdentTyped {
  pub ident: String,
  pub type_ident: IdentType,
  pub range: Range,
}

/// https://whistle.js.org/docs/specification/grammar#identifiers
#[derive(Debug, Clone, PartialEq)]
pub struct IdentImport {
  pub ident: String,
  pub as_ident: Option<String>,
  pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentExternFn {
  pub ident: String,
  pub params: Vec<IdentTyped>,
  pub ret_type: IdentType,
  pub range: Range,
}

/// https://whistle.js.org/docs/specification/grammar#expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
  Unary {
    unary: Unary,
    range: Range,
  },
  Binary {
    op: Operator,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    range: Range,
  },
  Cond {
    cond: Box<Expr>,
    then_expr: Box<Expr>,
    else_expr: Box<Expr>,
    range: Range,
  },
}

/// https://whistle.js.org/docs/specification/grammar#expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Unary {
  Primary {
    prim: Primary,
    range: Range,
  },
  UnaryOp {
    op: Operator,
    expr: Box<Unary>,
    range: Range,
  },
}

/// https://whistle.js.org/docs/specification/grammar#expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Primary {
  Literal {
    lit: Literal,
    range: Range,
  },
  IdentVal {
    ident: String,
    prim: Vec<IdentVal>,
    range: Range,
  },
  Grouping {
    group: Box<Expr>,
    range: Range,
  },
  Array {
    exprs: Vec<Expr>,
    type_ident: IdentType,
    range: Range,
  },
}

/// https://whistle.js.org/docs/specification/grammar#expressions
#[derive(Debug, Clone, PartialEq)]
pub enum IdentVal {
  Selector {
    ident: String,
    range: Range,
  },
  Arguments {
    args: Vec<Expr>,
    range: Range,
  },
  Index {
    expr: Box<Expr>,
    range: Range,
  },
  Slice {
    start: usize,
    end: usize,
    step: usize,
    range: Range,
  },
}

/// https://whistle.js.org/docs/specification/grammar#statements
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
  If {
    cond: Expr,
    then_stmt: Vec<Stmt>,
    else_stmt: Option<Vec<Stmt>>,
    range: Range,
  },
  While {
    cond: Expr,
    do_stmt: Vec<Stmt>,
    range: Range,
  },
  Continue {
    range: Range,
  },
  Break {
    range: Range,
  },
  Return {
    ret_type: Option<Expr>,
    range: Range,
  },
  VarDecl {
    ident_typed: IdentTyped,
    val: Expr,
    range: Range,
  },
  ValDecl {
    ident_typed: IdentTyped,
    val: Expr,
    range: Range,
  },
  Block {
    stmts: Vec<Stmt>,
    range: Range,
  },
  Tip {
    tip: Tip,
    range: Range,
  },
  Expr {
    expr: Expr,
    range: Range,
  },
  Assign {
    ident: String,
    op: Operator,
    rhs: Expr,
    range: Range,
  },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgramStmt {
  Import {
    idents: Vec<IdentImport>,
    from: String,
    imp_type: String,
    range: Range,
  },
  Extern {
    idents: Vec<IdentExternFn>,
    namespace: String,
    range: Range,
  },
  FunctionDecl {
    export: bool,
    inline: bool,
    ident: String,
    params: Vec<IdentTyped>,
    ret_type: IdentType,
    stmt: Vec<Stmt>,
    range: Range,
  },
  VarDecl {
    ident_typed: IdentTyped,
    val: Expr,
    range: Range,
  },
  ValDecl {
    ident_typed: IdentTyped,
    val: Expr,
    range: Range,
  },
  StructDecl {
    export: bool,
    ident: String,
    params: Vec<IdentTyped>,
    range: Range,
  },
  TypeDecl {
    export: bool,
    ident: String,
    types: IdentType,
    range: Range,
  },
  Stmt {
    stmt: Stmt,
    range: Range,
  },
}

/// https://whistle.js.org/docs/specification/grammar#grammar
pub type Grammar = Vec<ProgramStmt>;
