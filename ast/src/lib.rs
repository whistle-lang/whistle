pub use whistle_common::Literal;
pub use whistle_common::Operator;
pub use whistle_common::Primitive;
pub use whistle_common::Range;
pub use whistle_common::Tip;

#[derive(Debug, Clone, PartialEq)]
pub enum IdentType {
  Ident {
    ident: String,
    range: Option<Range>,
  },
  Generic {
    var: String,
    range: Option<Range>,
  },
  Var {
    var: usize,
    range: Option<Range>,
  },
  IdentType {
    ident: String,
    prim: Vec<IdentType>,
    range: Option<Range>,
  },
  Struct {
    ident: Vec<IdentTyped>,
    range: Option<Range>,
  },
  Primitive {
    prim: Primitive,
    range: Option<Range>,
  },
  Function {
    params: Vec<IdentTyped>,
    ret_type: Box<IdentType>,
    range: Option<Range>,
  },
  Array {
    ident: Box<IdentType>,
    range: Option<Range>,
  },
  Default,
  Error,
}

impl IdentType {
  pub fn to_type(&self) -> Type {
    match self {
      IdentType::Ident { ident, .. } => Type::Ident(ident.clone()),
      IdentType::Generic { var, .. } => Type::Generic(var.clone()),
      IdentType::Var { var, .. } => Type::Var(var.clone()),
      IdentType::IdentType { ident, prim, .. } => Type::IdentType {
        ident: ident.clone(),
        prim: IdentType::vec_to_type(prim),
      },
      IdentType::Struct { ident, .. } => Type::Struct(IdentTyped::vec_to_type(ident)),
      IdentType::Primitive { prim, .. } => Type::Primitive(prim.clone()),
      IdentType::Function {
        params, ret_type, ..
      } => Type::Function {
        params: IdentTyped::vec_to_type(params),
        ret_type: Box::new(ret_type.to_type()),
      },
      IdentType::Array { ident, .. } => Type::Array(Box::new(ident.to_type())),
      IdentType::Default => Type::Default,
      IdentType::Error => Type::Error,
    }
  }

  pub fn vec_to_type(types: &Vec<IdentType>) -> Vec<Type> {
    types.iter().map(|x| x.to_type()).collect()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Typed {
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
  Struct(Vec<Typed>),
  Primitive(Primitive),
  Function {
    params: Vec<Typed>,
    ret_type: Box<Type>,
  },
  Array(Box<Type>),
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
  pub range: Option<Range>,
}

impl IdentTyped {
  pub fn to_type(&self) -> Typed {
    Typed {
      ident: self.ident.clone(),
      type_ident: self.type_ident.to_type(),
    }
  }

  pub fn vec_to_type(types: &Vec<IdentTyped>) -> Vec<Typed> {
    types.iter().map(|x| x.to_type()).collect()
  }
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

impl Expr {
  pub fn range(&self) -> Range {
    match &self {
      Expr::Unary { range, .. } => range.clone(),
      Expr::Binary { range, .. } => range.clone(),
      Expr::Cond { range, .. } => range.clone()
    }
  }
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

impl Stmt {
  pub fn range(&self) -> Range {
    match &self {
      Stmt::If { range, .. } => range.clone(),
      Stmt::While { range, .. } => range.clone(),
      Stmt::Continue { range, .. } => range.clone(),
      Stmt::Break { range, .. } => range.clone(),
      Stmt::Return { range, .. } => range.clone(),
      Stmt::VarDecl { range, .. } => range.clone(),
      Stmt::ValDecl { range, .. } => range.clone(),
      Stmt::Block { range, .. } => range.clone(),
      Stmt::Tip { range, .. } => range.clone(),
      Stmt::Expr { range, .. } => range.clone(),
      Stmt::Assign { range, .. } => range.clone()
    }
  }
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
