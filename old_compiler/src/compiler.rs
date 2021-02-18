use crate::encoding::*;
use crate::opcodes::*;
use crate::types::*;

use std::collections::HashMap;

use wasm_encoder::TypeSection;
use wasm_encoder::ImportSection;
use wasm_encoder::FunctionSection;
use wasm_encoder::TableSection;
use wasm_encoder::MemorySection;
use wasm_encoder::GlobalSection;
use wasm_encoder::ExportSection;
use wasm_encoder::StartSection;
use wasm_encoder::ElementSection;
use wasm_encoder::CodeSection;
use wasm_encoder::DataSection;
use wasm_encoder::Module;

#[derive(Clone)]
pub struct Local {
  pub index: usize,
  pub local_type: Type,
}

impl Local {
  pub fn new(index: usize, local_type: Type) -> Local {
    Local { index, local_type }
  }
}

#[derive(Clone)]
pub struct Function {
  pub index: usize,
  pub export: bool,
  pub locals: HashMap<String, Local>,
  pub param_types: Vec<Type>,
  pub result_types: Vec<Type>,
  pub code: Vec<u8>,
}

impl Function {
  pub fn new() -> Function {
    Function {
      index: 0,
      export: false,
      locals: HashMap::new(),
      param_types: Vec::new(),
      result_types: Vec::new(),
      code: Vec::new(),
    }
  }
}

pub struct Context {
  name: String,
  types: TypeSection,
  imports: ImportSection,
  functions: FunctionSection,
  tables: TableSection,
  memories: MemorySection,
  globals: GlobalSection,
  exports: ExportSection,
  start: StartSection,
  elements: ElementSection,
  code: CodeSection,
  data: DataSection,
}

impl Context {
  pub fn finish(&self) -> Vec<u8> {
    let mut module = Module::new();
    module.section(&self.types);
    module.section(&self.imports);
    module.section(&self.functions);
    module.section(&self.tables);
    module.section(&self.memories);
    module.section(&self.globals);
    module.section(&self.exports);
    module.section(&self.start);
    module.section(&self.elements);
    module.section(&self.code);
    module.section(&self.data);
    module.finish()
  }
}

impl Compiler {
  pub fn new() -> Self {
    Self {
      strmem: Vec::new(),
      funcs: HashMap::new(),
      func: Function::new(),
      name: String::new(),
    }
  }

  pub fn set_param(&mut self, name: &str, param_type: Type) {
    let index = self.func.locals.len();
    self
      .func
      .locals
      .insert(name.to_string(), Local::new(index, param_type.clone()));
    self.func.param_types.push(param_type);
  }

  pub fn set_local(&mut self, name: &str, local_type: Type) {
    let len = self.func.locals.len();
    let offset = self.func.param_types.len();
    self
      .func
      .locals
      .insert(name.to_string(), Local::new(len + offset, local_type));
  }

  pub fn get_local(&mut self, name: &str) -> Local {
    if self.func.locals.contains_key(name) {
      if let Some(res) = self.func.locals.get(name) {
        return (*res).clone();
      }
    }

    panic!("Undefined local {}", name)
  }

  pub fn get_func(&mut self, name: &str) -> Function {
    if self.funcs.contains_key(name) {
      if let Some(res) = self.funcs.get(name) {
        return (*res).clone();
      }
    }

    panic!("Undefined function {}", name)
  }

  pub fn set_func(&mut self, name: &str, function: Function) {
    self.funcs.insert(name.to_string(), function);
  }
}

pub fn type_section(compiler: &mut Compiler) -> Vec<u8> {
  let mut res = vec![];
  for (_, func) in &compiler.funcs {
    let mut resfunc = vec![Names::FunctionType as u8];
    let params = func
      .param_types
      .clone()
      .into_iter()
      .map(|p| p.to_valtype() as u8)
      .collect();
    resfunc.extend(encode_vector(params));
    let types = func
      .result_types
      .clone()
      .into_iter()
      .map(|p| p.to_valtype() as u8)
      .collect();
    resfunc.extend(encode_vector(types));
    res.push(resfunc);
  }
  let body = encode_flatten(res);
  create_section(Section::Type, body)
}

pub fn global_section() -> Vec<u8> {
  let mut res = vec![ValType::I32 as u8];
  res.push(Names::Mut as u8);
  res.push(Opcode::I32Const as u8);
  res.extend(unsigned_leb128(0));
  res.push(Opcode::End as u8);
  let body = encode_flatten(vec![res]);
  create_section(Section::Global, body)
}

pub fn memory_section() -> Vec<u8> {
  let mut res = unsigned_leb128(1);
  res.extend(unsigned_leb128(1));
  res.extend(unsigned_leb128(1));
  let body = encode_flatten(vec![res]);
  create_section(Section::Memory, body)
}

pub fn encode_data(offset: usize, string: Vec<u8>) -> Vec<u8> {
  let mut res = vec![Names::EmptyArray as u8];
  res.push(Opcode::I32Const as u8);
  res.extend(unsigned_leb128(offset));
  res.push(Opcode::End as u8);
  res.extend(string.clone());
  res
}

pub fn data_section(compiler: &mut Compiler) -> Vec<u8> {
  let mut len = 100;
  let mut refs = vec![];
  for string in &compiler.strmem {
    refs.extend(vec![0, string.len() as u8]);
    refs.extend(double(unsigned_leb128(len)));
    len += string.len();
  }
  let mut res = vec![encode_data(100, encode_vector(refs))];
  res.push(encode_data(0, encode_string(&compiler.strmem.join(""))));
  let body = encode_flatten(res);
  create_section(Section::Data, body)
}

pub fn fun_section(compiler: &mut Compiler) -> Vec<u8> {
  let mut res = vec![];
  for i in 0..compiler.funcs.len() {
    res.push(unsigned_leb128(i));
  }
  let body = encode_flatten(res);
  create_section(Section::Func, body)
}

pub fn export_section(compiler: &mut Compiler) -> Vec<u8> {
  let mut code = vec![];
  for (name, fun) in compiler.funcs.iter() {
    if fun.export {
      let mut res = encode_string(name);
      res.push(ExportType::Func as u8);
      res.push(ExportType::Func as u8);
      let body = encode_flatten(vec![res]);
      code.push(create_section(Section::Export, body));
    }
  }
  encode_flatten(code)
}

pub fn code_section(compiler: &mut Compiler) -> Vec<u8> {
  let mut res = vec![];
  for (_, func) in &compiler.funcs {
    let mut code = encode_locals(func);
    code.extend(&func.code);
    code.push(Opcode::End as u8);
    res.push(encode_vector(code));
  }
  let body = encode_flatten(res);
  create_section(Section::Code, body)
}

pub fn compile_all(compiler: &mut Compiler) -> Vec<u8> {
  let mut header = MAGIC_MODULE_HEADER.to_vec();
  header.extend(MODULE_VERSION.to_vec());
  header.extend(type_section(compiler));
  header.extend(fun_section(compiler));
  header.extend(memory_section());
  header.extend(global_section());
  header.extend(export_section(compiler));
  header.extend(code_section(compiler));
  header.extend(data_section(compiler));
  header
}
