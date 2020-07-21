import { Program, FunctionDeclaration } from "../parser/program.ts";

export type Imports = { [module: string]: string[] | undefined };
export type Exports = FunctionDeclaration[];
export type External = (FunctionDeclaration | string)[];

export type CompilationFile = WhistleCompilationFile | JsCompilationFile;

export interface WhistleCompilationFile {
  language: "whistle";
  filename: string;
  program: Program;
}

export interface JsCompilationFile {
  language: "javascript";
  filename: string;
  content: string;
}

export interface CompilationSource extends WhistleCompilationFile {
  external: External;
}

export abstract class CompilationTarget<T> {
  abstract compile(source: CompilationSource): T;
}
