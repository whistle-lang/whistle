import { Program, FunctionDeclaration, CodeBlock } from "../parser/program.ts";
import { JsCompilationTarget } from "./target/js.ts";
import { CompilationFile, CompilationTarget, CompilationSource, Exports, Imports } from "./types.ts";

export class WhistleCompiler {
  public entry: CompilationFile;
  public files: CompilationFile[];
  public target: CompilationTarget<string>;

  constructor(
    entry: CompilationFile,
    files: CompilationFile[] = [],
    target: CompilationTarget<string> = new JsCompilationTarget(),
  ) {
    this.entry = entry;
    this.files = files;
    this.target = target;
  }

  public compile(): string {
    const external = this.findExternal(this.entry);
    const source: CompilationSource = { ...this.entry, external };

    return this.target.compile(source);
  }

  public findExternal(file: CompilationFile): Exports {
    const external: Set<FunctionDeclaration> = new Set();

    const fileImports = WhistleCompiler.findImports(file.program);

    for (const module in fileImports) {
      const importNames = fileImports[module];
      const importFile = this.findFile(module);

      if (!importFile) {
        throw `Could not find file ${importFile}`;
      }

      const fileExports = WhistleCompiler.findExports(importFile.program);
      const exportNames = fileExports.map((f) => f.value.name);

      for (const importName of importNames) {
        if (!exportNames.includes(importName)) {
          throw `${importFile} does not export ${importName}`;
        }

        for (const fileExport of fileExports) {
          if (fileExport.value.name === importName) {
            external.add(fileExport);
          }
        }
      }

      let externalImports = this.findExternal(importFile);

      for (const externalImport of externalImports) {
        external.add(externalImport);
      }
    }

    return [...external.values()];
  }

  public findFile(name: string): CompilationFile | undefined {
    return this.files.find((f) => f.filename === name);
  }

  public static findImports(program: Program): Imports {
    const imports: Imports = {};

    for (const statement of program.value) {
      if (statement.type === "ImportDeclaration") {
        imports[statement.value.module.value] = statement.value.names ?? [];
      }
    }

    return imports;
  }

  public static findExports(program: Program): Exports {
    const exports: Exports = [];

    for (const statement of program.value) {
      if (
        statement.type === "FunctionDeclaration" && statement.value.exported
      ) {
        exports.push(statement);
      }
    }

    return exports;
  }
}
