import { Program, ImportDeclaration, FunctionDeclaration, CodeBlock } from "../parser/program.ts";
import { JsCompilationTarget } from "./js/target.ts";
import { Operator } from "../parser/operator.ts";
import { Statement } from "../parser/statement.ts";
import { Expression } from "../parser/expression.ts";

export interface CompilationTarget<T> {
  CompileFunctionDeclaration(declaration: FunctionDeclaration): T;
  CompileCodeBlock(block: CodeBlock): T;
  Comment(text: string): T;
  CompileStatement(statement: Statement): T;
  CompileOperator(operator: Operator): T;
  CompileExpression(expression: Expression): T;
}

export interface CompilationFile {
  filename: string;
  program: Program;
}

export type Imports = { [module: string]: string[] };
export type Exports = FunctionDeclaration[];

export interface CompilationSource extends CompilationFile {
  imports: Imports;
  exports: Exports;
}

export class WhistleCompiler {
  public readonly entry: CompilationFile;
  public readonly files: CompilationFile[];
  public readonly target: CompilationTarget<any>;

  constructor(entry: CompilationFile, files: CompilationFile[] = [], target: CompilationTarget<any> = JsCompilationTarget) {
    this.entry = entry;
    this.files = files;
    this.target = target;
  }

  public compile(): string {
    const source = this.analyze(this.entry);
    const required: Set<FunctionDeclaration> = new Set();

    let result = "";

    for (const module in source.imports) {
      const names = source.imports[module];
      const file = this.file(module);

      if (!file) {
        throw `Could not find file ${module}`;
      }

      const exports = this.exports(file.program);

      if (names.length === 0) {
        for (const exported of exports) {
          required.add(exported);
        }
      } else {
        for (const exported of exports) {
          if (names.includes(exported.value.name)) {
            required.add(exported);
          }
        }
      }
    }

    for (const func of required) {
      result += this.target.CompileFunctionDeclaration(func);
    }

    for (const statement of source.program.value) {
      switch (statement.type) {
        case "FunctionDeclaration":
          result += this.target.CompileFunctionDeclaration(statement);
          break;

        case "CodeBlock":
          result += this.target.CompileCodeBlock(statement);
          break;
      }
    }

    return result;
  }

  private file(name: string): CompilationFile | undefined {
    return this.files.find(f => f.filename === name);
  }

  private analyze(file: CompilationFile): CompilationSource {
    return { ...file, imports: this.imports(file.program), exports: this.exports(file.program) };
  }

  private imports(program: Program): Imports {
    const imports: Imports = {};

    for (const statement of program.value) {
      if (statement.type === "ImportDeclaration") {
        imports[statement.value.module.value] = statement.value.names ?? [];
      }
    }

    return imports;
  }

  private exports(program: Program): Exports {
    const exports: Exports = [];

    for (const statement of program.value) {
      if (statement.type === "FunctionDeclaration" && statement.value.exported) {
        exports.push(statement);
      }
    }

    return exports;
  }
}
