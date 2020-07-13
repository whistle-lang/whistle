import { Program, ImportDeclaration } from "../parser/program.ts";

export enum CompilationTarget {
  JS,
  // WASM32,
}

export interface CompilationSource {
  [file: string]: {
    program: Program;
    imports: {
      names?: string[];
      module: string;
    };
    exports: string[];
    entry: boolean;
  }
}

export class WhistleCompiler {
  public readonly target: CompilationTarget;
  public readonly source: CompilationSource;

  constructor(source: CompilationSource, target: CompilationTarget = CompilationTarget.JS) {
    this.source = source;
    this.target = target;
  }
}
