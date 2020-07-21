import { Program, FunctionDeclaration } from "../parser/program.ts";

export type Imports = { [module: string]: string[] };
export type Exports = FunctionDeclaration[];

export interface CompilationFile {
  filename: string;
  program: Program;
}

export interface CompilationSource extends CompilationFile {
  external: Exports;
}

export abstract class CompilationTarget<T> {
  abstract compile(source: CompilationSource): T;
}
