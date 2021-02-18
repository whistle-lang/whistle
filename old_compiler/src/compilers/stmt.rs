use crate::compiler::Compiler;
use crate::compilers::compile_expr;
use crate::encoding::signed_leb128;
use crate::encoding::unsigned_leb128;
use crate::opcodes::Blocktype;
use crate::opcodes::Opcode;
use crate::types::Type;

use whistle_ast::Expr;
use whistle_ast::IdentTyped;
use whistle_ast::Operator;
use whistle_ast::Stmt;
use whistle_ast::Tip;

pub fn compile_stmt(compiler: &mut Compiler, stmt: Stmt) {
  match stmt {
    // Stmt::Tip(args) => compile_tip(args),
    Stmt::While { cond, do_stmt } => compile_while(compiler, cond, *do_stmt),
    Stmt::ValDecl { ident_typed, val } => compile_var_decl(compiler, ident_typed, *val),
    Stmt::VarDecl { ident_typed, val } => compile_var_decl(compiler, ident_typed, *val),
    // Stmt::Assign { op, rhs, ident } => compile_assign(compiler, op, *rhs, ident),
    Stmt::If {
      cond,
      then_stmt,
      else_stmt,
    } => compile_if(compiler, *cond, *then_stmt, else_stmt),
    Stmt::Expr(args) => compile_expr_stmt(compiler, args),
    Stmt::Block(args) => compile_block(compiler, args),
    _ => panic!("stmt"),
  }
}

pub fn compile_tip(_tip: Tip) {}

pub fn compile_while(compiler: &mut Compiler, cond: Option<Box<Expr>>, do_stmt: Stmt) {
  compiler.func.code.push(Opcode::Block as u8);
  compiler.func.code.push(Blocktype::Empty as u8);
  compiler.func.code.push(Opcode::Loop as u8);
  compiler.func.code.push(Blocktype::Empty as u8);
  if let Some(expr) = cond {
    compile_expr(compiler, *expr);
    compiler.func.code.push(Opcode::BrIf as u8);
    compiler.func.code.extend(signed_leb128(1));
  }
  compile_stmt(compiler, do_stmt);
  compiler.func.code.push(Opcode::Br as u8);
  compiler.func.code.extend(signed_leb128(0));
  compiler.func.code.push(Opcode::End as u8);
  compiler.func.code.push(Opcode::End as u8);
}

pub fn compile_if(
  compiler: &mut Compiler,
  cond: Expr,
  then_stmt: Stmt,
  else_stmt: Option<Box<Stmt>>,
) {
  compiler.func.code.push(Opcode::Block as u8);
  compiler.func.code.push(Blocktype::Empty as u8);
  compile_expr(compiler, cond.clone());
  compiler.func.code.push(Opcode::I32Eqz as u8);
  compiler.func.code.push(Opcode::BrIf as u8);
  compiler.func.code.extend(signed_leb128(0));
  compile_stmt(compiler, then_stmt);
  compiler.func.code.push(Opcode::End as u8);

  if let Some(stmts) = else_stmt {
    compiler.func.code.push(Opcode::Block as u8);
    compiler.func.code.push(Blocktype::Empty as u8);
    compile_expr(compiler, cond);
    compiler.func.code.push(Opcode::I32Const as u8);
    compiler.func.code.extend(signed_leb128(1));
    compiler.func.code.push(Opcode::I32Eq as u8);
    compiler.func.code.push(Opcode::BrIf as u8);
    compiler.func.code.extend(signed_leb128(0));
    compile_stmt(compiler, *stmts);
    compiler.func.code.push(Opcode::End as u8);
  }
}

pub fn compile_var_decl(compiler: &mut Compiler, ident: IdentTyped, val: Expr) {
  let type1 = compile_expr(compiler, val);
  let type2 = Type::from(&ident.type_ident);
  if Type::is_compat(&type1, &type2) {
    if let Ok(Some(conversion)) = type1.convert(&type2) {
      compiler.func.code.push(conversion as u8);
    }
  } else {
    panic!("Cannot cast type! {:?} -> {:?}", type1, type2)
  }
  compiler.func.code.push(Opcode::LocalSet as u8);
  compiler.set_local(&ident.ident, type2);
  for elem in unsigned_leb128(compiler.get_local(&ident.ident).index) {
    compiler.func.code.push(elem);
  }
}

pub fn compile_block(compiler: &mut Compiler, stmts: Vec<Stmt>) {
  for stmt in stmts {
    compile_stmt(compiler, stmt)
  }
}

pub fn compile_assign(compiler: &mut Compiler, _op: Operator, rhs: Expr, ident: String) {
  //TODO: more assignment operators
  let type1 = compile_expr(compiler, rhs);
  compiler.func.code.push(Opcode::LocalSet as u8);
  let local = compiler.get_local(&ident);
  if Type::is_compat(&type1, &local.local_type) {
    panic!("Mismatched types!")
  }
  compiler.func.code.extend(unsigned_leb128(local.index));
}

pub fn compile_expr_stmt(compiler: &mut Compiler, expr: Expr) {
  compile_expr(compiler, expr);
}
