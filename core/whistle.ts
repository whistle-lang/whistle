import { WhistleTokenizer } from "./parser/tokenizer.ts";
import { WhistleParser } from "./parser/parser.ts";
import { WhistleCompiler } from "./compiler/compiler.ts";
import { Token } from "https://deno.land/x/tokenizer/token.ts";
import { Program, ProgramStatement } from "./parser/program.ts";

export class Whistle {
  private tokenizer: WhistleTokenizer;

  constructor() {
    this.tokenizer = new WhistleTokenizer();
  }

  private isProgram(value: any): value is Program {
    return Array.isArray(value) && value.length >= 1 &&
      value.every((item) => item instanceof ProgramStatement);
  }

  public tokenize(source: string): Token[] {
    return this.tokenizer.tokenize(source);
  }

  public parse(source: string): Program;
  public parse(tokens: Token[]): Program;
  public parse(tokensOrSource: Token[] | string): Program {
    if (typeof tokensOrSource === "string") {
      return new WhistleParser(this.tokenize(tokensOrSource)).parse();
    } else {
      return new WhistleParser(tokensOrSource).parse();
    }
  }

  public compile<T extends new(p: Program) => WhistleCompiler>(
    compiler: T,
    source: string,
  ): string;
  public compile<T extends new(p: Program) => WhistleCompiler>(
    compiler: T,
    tokens: Token[],
  ): string;
  public compile<T extends new(p: Program) => WhistleCompiler>(
    compiler: T,
    program: Program,
  ): string;
  public compile<T extends new(p: Program) => WhistleCompiler>(
    compiler: T,
    sourceTokensOrProgram: string | Token[] | Program,
  ): string {
    return new compiler(
      this.isProgram(sourceTokensOrProgram)
        ? sourceTokensOrProgram
        : this.parse(sourceTokensOrProgram),
    ).compile();
  }
}
