use wasm_encoder::CodeSection;
use wasm_encoder::DataSection;
use wasm_encoder::ElementSection;
use wasm_encoder::ExportSection;
use wasm_encoder::FunctionSection;
use wasm_encoder::GlobalSection;
use wasm_encoder::ImportSection;
use wasm_encoder::MemorySection;
use wasm_encoder::TableSection;
use wasm_encoder::TypeSection;

pub struct Module {
  pub types: TypeSection,
  pub imports: ImportSection,
  pub funs: FunctionSection,
  pub tables: TableSection,
  pub memories: MemorySection,
  pub globals: GlobalSection,
  pub exports: ExportSection,
  pub elements: ElementSection,
  pub code: CodeSection,
  pub data: DataSection,
}

impl Module {
  pub fn new() -> Self {
    Self {
      types: TypeSection::new(),
      imports: ImportSection::new(),
      funs: FunctionSection::new(),
      tables: TableSection::new(),
      memories: MemorySection::new(),
      globals: GlobalSection::new(),
      exports: ExportSection::new(),
      elements: ElementSection::new(),
      code: CodeSection::new(),
      data: DataSection::new(),
    }
  }

  pub fn finish(&self) -> Vec<u8> {
    let mut module = wasm_encoder::Module::new();
    module.section(&self.types);
    module.section(&self.imports);
    module.section(&self.funs);
    module.section(&self.tables);
    module.section(&self.memories);
    module.section(&self.globals);
    module.section(&self.exports);
    module.section(&self.elements);
    module.section(&self.code);
    module.section(&self.data);
    module.finish()
  }
}

impl Default for Module {
  fn default() -> Self {
    Module::new()
  }
}
