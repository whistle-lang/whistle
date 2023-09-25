use crate::compile_expr;
use crate::compile_tip_wasm_bytes;
use crate::ident_type_to_val_type;
use crate::Compiler;
use crate::Function;
use crate::IndexedSymbol;
use crate::Symbol;
use whistle_common::CompilerErrorKind;
use whistle_common::CompilerHandler;
use whistle_common::Span;

use wasm_encoder::BlockType;
use wasm_encoder::Instruction;

use whistle_ast::Expr;
use whistle_ast::IdentTyped;
use whistle_ast::Stmt;
use whistle_common::Tip;

pub fn compile_stmt(compiler: &mut Compiler, function: &mut Function, stmt: Stmt) {
  match stmt {
    Stmt::While { cond, do_stmt, .. } => compile_while(compiler, function, cond, do_stmt),
    Stmt::ValDecl {
      ident_typed, val, ..
    } => compile_val_decl(compiler, function, ident_typed, val),
    Stmt::Assign { rhs, ident, span } => compile_assign(compiler, function, rhs, ident, span),
    Stmt::VarDecl {
      ident_typed, val, ..
    } => compile_var_decl(compiler, function, ident_typed, val),
    Stmt::If {
      cond,
      then_stmt,
      else_stmt,
      ..
    } => compile_if(compiler, function, cond, then_stmt, else_stmt),
    Stmt::Tip { tip, span } => compile_tip(compiler, function, tip, span),
    Stmt::Expr { expr, .. } => compile_expr_stmt(compiler, function, expr),
    Stmt::Block { stmts, .. } => compile_block(compiler, function, stmts),
    Stmt::Return { ret_type, .. } => compile_return(compiler, function, ret_type),
    _ => compiler
      .handler
      .throw(CompilerErrorKind::Unimplemented, stmt.span()),
  }
}

pub fn compile_stmts(compiler: &mut Compiler, function: &mut Function, stmts: Vec<Stmt>) {
  compiler.scope.enter_curr_scope();
  for stmt in stmts {
    compile_stmt(compiler, function, stmt);
  }
  compiler.scope.exit_scope();
}

pub fn compile_while(
  compiler: &mut Compiler,
  function: &mut Function,
  cond: Expr,
  do_stmt: Vec<Stmt>,
) {
  function.instruction(Instruction::Block(BlockType::Empty));
  function.instruction(Instruction::Loop(BlockType::Empty));
  compile_expr(compiler, function, cond);
  function.instruction(Instruction::BrIf(1));
  compile_stmts(compiler, function, do_stmt);
  function.instruction(Instruction::Br(0));
  function.instruction(Instruction::End);
  function.instruction(Instruction::End);
}

pub fn compile_if(
  compiler: &mut Compiler,
  function: &mut Function,
  cond: Expr,
  then_stmt: Vec<Stmt>,
  else_stmt: Option<Vec<Stmt>>,
) {
  compile_expr(compiler, function, cond);
  function.instruction(Instruction::If(BlockType::Empty));
  compile_stmts(compiler, function, then_stmt);

  if let Some(stmt) = else_stmt {
    function.instruction(Instruction::Else);
    compile_stmts(compiler, function, stmt);
  }

  function.instruction(Instruction::End);
}

pub fn compile_val_decl(
  compiler: &mut Compiler,
  function: &mut Function,
  ident: IdentTyped,
  val: Expr,
) {
  let types = compile_expr(compiler, function, val);
  let sym = compiler.get_sym(&ident.ident).unwrap();
  function.local(sym.0, ident_type_to_val_type(types));
  function.instruction(Instruction::LocalSet(sym.0));
}

pub fn compile_var_decl(
  compiler: &mut Compiler,
  function: &mut Function,
  ident: IdentTyped,
  val: Expr,
) {
  let types = compile_expr(compiler, function, val);
  let sym = compiler.get_sym(&ident.ident).unwrap();
  function.local(sym.0, ident_type_to_val_type(types));
  function.instruction(Instruction::LocalSet(sym.0));
}

pub fn compile_block(compiler: &mut Compiler, function: &mut Function, stmts: Vec<Stmt>) {
  compiler.scope.enter_curr_scope();
  function.instruction(Instruction::Loop(BlockType::Empty));
  for stmt in stmts {
    compile_stmt(compiler, function, stmt)
  }
  function.instruction(Instruction::End);
  compiler.scope.exit_scope();
}

pub fn compile_return(compiler: &mut Compiler, function: &mut Function, expr: Option<Expr>) {
  if let Some(expr) = expr {
    compile_expr(compiler, function, expr);
  }
  function.instruction(Instruction::Return);
}

pub fn compile_assign(
  compiler: &mut Compiler,
  function: &mut Function,
  rhs: Expr,
  ident: String,
  span: Span,
) {
  let sym = match compiler.get_sym(&ident) {
    Ok(sym) => sym.clone(),
    Err(err) => {
      compiler.handler.throw(err, span);
      IndexedSymbol(0, Symbol::default())
    }
  };
  compile_expr(compiler, function, rhs);

  if sym.1.global {
    function.instruction(Instruction::GlobalSet(sym.0));
  } else {
    function.instruction(Instruction::LocalSet(sym.0));
  }
}

pub fn compile_expr_stmt(compiler: &mut Compiler, function: &mut Function, expr: Expr) {
  compile_expr(compiler, function, expr);
}

pub fn compile_tip(compiler: &mut Compiler, function: &mut Function, tip: Tip, span: Span) {
  match tip.ident.as_str() {
    "wasm_bytes" => compile_tip_wasm_bytes(compiler, function, tip, span),
    _ => compiler
      .handler
      .throw(CompilerErrorKind::Unimplemented, span),
  }
}
