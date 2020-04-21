import { Program } from "../parser/program.ts";

export abstract class Compiler {
  protected program: Program;

  constructor(program: Program) {
    this.program = program;
  }

  public abstract compile(): string;
}

// very abstract class
// mmm yes