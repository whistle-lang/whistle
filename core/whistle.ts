import { Token } from "./deps.ts";
import { WhistleTokenizer } from "./parser/tokenizer.ts";
import { WhistleParser } from "./parser/parser.ts";
import { WhistleCompiler } from "./compiler/compiler.ts";
import { Program, ParseProgram } from "./parser/program.ts";

export class Whistle {
  private tokenizer: WhistleTokenizer;

  constructor() {
    this.tokenizer = new WhistleTokenizer();
  }

  public tokenize(source: string): Token[] {
    return this.tokenizer.tokenize(source);
  }

  public parse(source: string): Program;
  public parse(tokens: Token[]): Program;
  public parse(tokensOrSource: Token[] | string): Program {
    const parser = new WhistleParser(
      typeof tokensOrSource === "string"
        ? this.tokenize(tokensOrSource)
        : tokensOrSource,
    );

    return ParseProgram(parser);
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
    let program: Program;

    if (
      typeof sourceTokensOrProgram === "string"
    ) {
      program = this.parse(sourceTokensOrProgram);
    } else if (sourceTokensOrProgram instanceof Array) {
      program = this.parse(sourceTokensOrProgram);
    } else {
      program = sourceTokensOrProgram;
    }

    return new compiler(program).compile();
  }
}
