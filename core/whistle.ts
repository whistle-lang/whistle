import { Token } from "./deps.ts";
import { WhistleTokenizer } from "./parser/tokenizer.ts";
import { WhistleParser } from "./parser/parser.ts";
import { WhistleCompiler } from "./compiler/compiler.ts";
import { Program, ProgramStatement } from "./parser/program.ts";

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
      sourceTokensOrProgram instanceof Program
        ? sourceTokensOrProgram
        : this.parse(sourceTokensOrProgram as any),
    ).compile();
  }
}
