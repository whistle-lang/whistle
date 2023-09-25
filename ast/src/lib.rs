pub use whistle_common::Literal;
pub use whistle_common::Operator;
pub use whistle_common::Primitive;
pub use whistle_common::Span;
pub use whistle_common::Tip;
pub use whistle_common::Type;
pub use whistle_common::TypedIdent;

#[derive(Debug, Clone, PartialEq)]
pub enum IdentType {
  Ident {
    ident: String,
    span: Option<Span>,
  },
  Generic {
    var: String,
    span: Option<Span>,
  },
  Var {
    var: usize,
    span: Option<Span>,
  },
  IdentType {
    ident: String,
    prim: Vec<IdentType>,
    span: Option<Span>,
  },
  Struct {
    ident: Vec<IdentTyped>,
    span: Option<Span>,
  },
  Primitive {
    prim: Primitive,
    span: Option<Span>,
  },
  Function {
    params: Vec<IdentTyped>,
    ret_type: Box<IdentType>,
    span: Option<Span>,
  },
  Array {
    ident: Box<IdentType>,
    span: Option<Span>,
  },
  Default,
  Error,
}

impl IdentType {
  pub fn to_type(&self) -> Type {
    match self {
      IdentType::Ident { ident, .. } => Type::Ident(ident.clone()),
      IdentType::Generic { var, .. } => Type::Generic(var.clone()),
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
      _ => unreachable!(),
    }
  }

  pub fn vec_to_type(types: &Vec<IdentType>) -> Vec<Type> {
    types.iter().map(|x| x.to_type()).collect()
  }
}

/// https://whistle.js.org/docs/specification/grammar#identifiers
#[derive(Debug, Clone, PartialEq)]
pub struct IdentFunction {
  pub ident: String,
  pub generic: Vec<String>,
  pub span: Span,
}

/// https://whistle.js.org/docs/specification/grammar#identifiers
#[derive(Debug, Clone, PartialEq)]
pub struct IdentTyped {
  pub ident: String,
  pub type_ident: IdentType,
  pub span: Option<Span>,
}

impl IdentTyped {
  pub fn to_type(&self) -> TypedIdent {
    TypedIdent {
      ident: self.ident.clone(),
      type_ident: self.type_ident.to_type(),
    }
  }

  pub fn vec_to_type(types: &Vec<IdentTyped>) -> Vec<TypedIdent> {
    types.iter().map(|x| x.to_type()).collect()
  }
}

/// https://whistle.js.org/docs/specification/grammar#identifiers
#[derive(Debug, Clone, PartialEq)]
pub struct IdentImport {
  pub ident: String,
  pub as_ident: Option<String>,
  pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentExternFn {
  pub ident: String,
  pub params: Vec<IdentTyped>,
  pub ret_type: IdentType,
  pub span: Span,
}

/// https://whistle.js.org/docs/specification/grammar#expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
  Unary {
    unary: Unary,
    span: Span,
  },
  Binary {
    op: Operator,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    span: Span,
  },
  Cond {
    cond: Box<Expr>,
    then_expr: Box<Expr>,
    else_expr: Box<Expr>,
    span: Span,
  },
}

impl Expr {
  pub fn span(&self) -> Span {
    match &self {
      Expr::Unary { span, .. } => span.clone(),
      Expr::Binary { span, .. } => span.clone(),
      Expr::Cond { span, .. } => span.clone(),
    }
  }
}

/// https://whistle.js.org/docs/specification/grammar#expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Unary {
  Primary {
    prim: Primary,
    span: Span,
  },
  UnaryOp {
    op: Operator,
    expr: Box<Unary>,
    span: Span,
  },
}

/// https://whistle.js.org/docs/specification/grammar#expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Primary {
  Literal {
    lit: Literal,
    meta_id: usize,
    span: Span,
  },
  IdentVal {
    ident: String,
    prim: Vec<IdentVal>,
    span: Span,
  },
  Grouping {
    group: Box<Expr>,
    span: Span,
  },
  Array {
    exprs: Vec<Expr>,
    meta_id: usize,
    span: Span,
  },
}

/// https://whistle.js.org/docs/specification/grammar#expressions
#[derive(Debug, Clone, PartialEq)]
pub enum IdentVal {
  Selector {
    ident: String,
    span: Span,
  },
  Arguments {
    args: Vec<Expr>,
    span: Span,
  },
  Index {
    expr: Box<Expr>,
    span: Span,
  },
  Slice {
    start: usize,
    end: usize,
    step: usize,
    span: Span,
  },
}

impl IdentVal {
  pub fn span(&self) -> Span {
    match &self {
      IdentVal::Selector { span, .. } => span.clone(),
      IdentVal::Arguments { span, .. } => span.clone(),
      IdentVal::Index { span, .. } => span.clone(),
      IdentVal::Slice { span, .. } => span.clone(),
    }
  }
}

/// https://whistle.js.org/docs/specification/grammar#statements
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
  If {
    cond: Expr,
    then_stmt: Vec<Stmt>,
    else_stmt: Option<Vec<Stmt>>,
    span: Span,
  },
  While {
    cond: Expr,
    do_stmt: Vec<Stmt>,
    span: Span,
  },
  Continue {
    span: Span,
  },
  Break {
    span: Span,
  },
  Return {
    ret_type: Option<Expr>,
    span: Span,
  },
  VarDecl {
    ident_typed: IdentTyped,
    val: Expr,
    span: Span,
  },
  ValDecl {
    ident_typed: IdentTyped,
    val: Expr,
    span: Span,
  },
  Block {
    stmts: Vec<Stmt>,
    span: Span,
  },
  Tip {
    tip: Tip,
    span: Span,
  },
  Expr {
    expr: Expr,
    span: Span,
  },
  Assign {
    ident: String,
    rhs: Expr,
    span: Span,
  },
}

impl Stmt {
  pub fn span(&self) -> Span {
    match &self {
      Stmt::If { span, .. } => span.clone(),
      Stmt::While { span, .. } => span.clone(),
      Stmt::Continue { span, .. } => span.clone(),
      Stmt::Break { span, .. } => span.clone(),
      Stmt::Return { span, .. } => span.clone(),
      Stmt::VarDecl { span, .. } => span.clone(),
      Stmt::ValDecl { span, .. } => span.clone(),
      Stmt::Block { span, .. } => span.clone(),
      Stmt::Tip { span, .. } => span.clone(),
      Stmt::Expr { span, .. } => span.clone(),
      Stmt::Assign { span, .. } => span.clone(),
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProgramStmt {
  Import {
    idents: Vec<IdentImport>,
    from: String,
    imp_type: String,
    span: Span,
  },
  Extern {
    idents: Vec<IdentExternFn>,
    namespace: String,
    span: Span,
  },
  FunctionDecl {
    export: bool,
    inline: bool,
    ident: String,
    params: Vec<IdentTyped>,
    ret_type: IdentType,
    stmt: Vec<Stmt>,
    span: Span,
  },
  VarDecl {
    ident_typed: IdentTyped,
    val: Expr,
    span: Span,
  },
  ValDecl {
    ident_typed: IdentTyped,
    val: Expr,
    span: Span,
  },
  StructDecl {
    export: bool,
    ident: String,
    params: Vec<IdentTyped>,
    span: Span,
  },
  TypeDecl {
    export: bool,
    ident: String,
    types: IdentType,
    span: Span,
  },
  Stmt {
    stmt: Stmt,
    span: Span,
  },
  Tip {
    tip: Tip,
    span: Span,
  },
}

impl ProgramStmt {
  pub fn span(&self) -> Span {
    match &self {
      ProgramStmt::Import { span, .. } => span.clone(),
      ProgramStmt::Extern { span, .. } => span.clone(),
      ProgramStmt::FunctionDecl { span, .. } => span.clone(),
      ProgramStmt::VarDecl { span, .. } => span.clone(),
      ProgramStmt::ValDecl { span, .. } => span.clone(),
      ProgramStmt::StructDecl { span, .. } => span.clone(),
      ProgramStmt::TypeDecl { span, .. } => span.clone(),
      ProgramStmt::Stmt { span, .. } => span.clone(),
      ProgramStmt::Tip { span, .. } => span.clone(),
    }
  }
}

/// https://whistle.js.org/docs/specification/grammar#grammar
pub type Grammar = Vec<ProgramStmt>;
