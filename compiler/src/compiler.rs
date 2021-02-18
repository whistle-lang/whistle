use crate::CompilerError;
use crate::CompilerErrorKind;
use crate::Scope;
use crate::SymbolTable;

pub struct Compiler {
	pub scope: Scope,
	pub table: SymbolTable,
	pub errors: Vec<CompilerError>,
}

impl Compiler {
	pub fn throw(&mut self, error: CompilerErrorKind, index: usize) {
		self.errors.push(CompilerError::new(error, index))
	}

	pub fn new_scope(&mut self) -> Scope {
		let scope = Scope::new(Some(self.scope));
		self.scope = scope;
		scope
	}

	pub fn exit_scope(&mut self) {
		if let Some(scope) = *self.scope.parent {
			self.scope = scope
		}
	}
}
