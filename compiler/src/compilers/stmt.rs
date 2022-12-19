use crate::compile_expr;
use crate::ident_type_to_val_type;
use crate::Compiler;
use crate::Function;
use crate::IndexedSymbol;
use crate::Symbol;
use whistle_common::Range;

use wasm_encoder::BlockType;
use wasm_encoder::Instruction;

use whistle_ast::Expr;
use whistle_ast::IdentTyped;
use whistle_ast::Stmt;

pub fn compile_stmt(compiler: &mut Compiler, function: &mut Function, stmt: Stmt) {
  match stmt {
    Stmt::While { cond, do_stmt, .. } => compile_while(compiler, function, cond, do_stmt),
    Stmt::ValDecl {
      ident_typed,
      val,
      range,
    } => compile_val_decl(compiler, function, ident_typed, val, range),
    Stmt::Assign { rhs, ident, range } => compile_assign(compiler, function, rhs, ident, range),
    Stmt::VarDecl {
      ident_typed,
      val,
      range,
    } => compile_var_decl(compiler, function, ident_typed, val, range),
    Stmt::If {
      cond,
      then_stmt,
      else_stmt,
      ..
    } => compile_if(compiler, function, cond, then_stmt, else_stmt),
    Stmt::Expr { expr, .. } => compile_expr_stmt(compiler, function, expr),
    Stmt::Block { stmts, .. } => compile_block(compiler, function, stmts),
    Stmt::Return { ret_type, .. } => compile_return(compiler, function, ret_type),
    _ => unimplemented!(),
  }
}

pub fn compile_stmts(compiler: &mut Compiler, function: &mut Function, stmts: Vec<Stmt>) {
  compiler.scope.enter_scope();
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
  range: Range,
) {
  let types = compile_expr(compiler, function, val);

  let idx = match compiler.scope.set_local_sym(
    &ident.ident,
    Symbol {
      global: false,
      mutable: true,
      types: ident.type_ident.to_type(),
    },
  ) {
    Ok(idx) => idx,
    Err(err) => {
      compiler.throw(err, range);
      0
    }
  };

  function.local(idx, ident_type_to_val_type(types));
  function.instruction(Instruction::LocalSet(idx));
}

pub fn compile_var_decl(
  compiler: &mut Compiler,
  function: &mut Function,
  ident: IdentTyped,
  val: Expr,
  range: Range,
) {
  let types = compile_expr(compiler, function, val);

  let idx = match compiler.scope.set_local_sym(
    &ident.ident,
    Symbol {
      global: false,
      mutable: false,
      types: ident.type_ident.to_type(),
    },
  ) {
    Ok(idx) => idx,
    Err(err) => {
      compiler.throw(err, range);
      0
    }
  };

  function.local(idx, ident_type_to_val_type(types));
  function.instruction(Instruction::LocalSet(idx));
}

pub fn compile_block(compiler: &mut Compiler, function: &mut Function, stmts: Vec<Stmt>) {
  compiler.scope.enter_scope();
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
  range: Range,
) {
  let sym = match compiler.scope.get_sym(&ident) {
    Ok(sym) => sym.clone(),
    Err(err) => {
      compiler.throw(err, range);
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
