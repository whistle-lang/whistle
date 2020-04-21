import { Program } from "../parser/program.ts";

export abstract class WhistleCompiler {
  protected program: Program;

  constructor(program: Program) {
    this.program = program;
  }

  public abstract compile(): string;
}
