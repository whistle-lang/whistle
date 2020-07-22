import { Token } from "../tokenizer/token.ts";

export class WhistleParser {
  private tokens: Token[];
  private index: number;

  constructor(tokens: Token[]) {
    this.tokens = tokens;
    this.index = 0;
  }

  public get current(): Token {
    return this.tokens[this.index];
  }

  public get next(): Token {
    return this.tokens[this.index + 1];
  }

  public at(index: number): Token {
    return this.tokens[index];
  }

  public offset(offset: number): Token {
    return this.tokens[this.index + offset];
  }

  public advance(steps: number = 1): Token {
    this.index += steps;
    return this.offset(-steps);
  }

  public eat(
    { value, type }: { value?: string; type?: string },
    token: Token = this.current,
  ): Token {
    if (!token) {
      throw `Unexpected empty token`;
    }

    if (type && value) {
      if (token.type !== type && token.value !== value) {
        throw `Expected type "${type}" with value "${value}" but recieved type "${token.type}" with value "${token.value}" at index ${token.position.start}`;
      }

      if (token.type !== type) {
        throw `Expected type "${type}" but recieved type "${token.type}" at index ${token.position.start}`;
      }

      if (token.value !== value) {
        throw `Expected value "${value}" but recieved "${token.value}" at index ${token.position.start}`;
      }
    } else {
      if (type) {
        if (token.type !== type) {
          throw `Expected type "${type}" but recieved type "${token.type}" at index ${token.position.start}`;
        }
      }
      if (value) {
        if (token.value !== value) {
          throw `Expected value "${value}" but recieved value "${token.value}" at index ${token.position.start}`;
        }
      }
    }

    return this.advance();
  }

  public is(
    { value, type }: { value?: string; type?: string },
    token: Token = this.current,
  ): boolean {
    if (!token) {
      throw `Unexpected empty token`;
    }

    if (type && value) {
      return token.type === type && token.value === value;
    } else {
      if (type) {
        return token.type === type;
      }

      if (value) {
        return token.value === value;
      }
    }

    return false;
  }

  public delimited<T>(
    start: { value?: string; type?: string },
    stop: { value?: string; type?: string },
    separator: { value?: string; type?: string },
    parser: () => T,
  ): T[] {
    const nodes = [];

    this.eat(start);

    while (!this.is(stop)) {
      nodes.push(parser());
      if (!this.is(stop)) {
        this.eat(separator);
      }
    }

    this.eat(stop);

    return nodes;
  }

  public until<T>(
    stop: { value?: string; type?: string },
    separator: { value?: string; type?: string },
    parser: () => T,
  ): T[] {
    const nodes = [];

    while (!this.is(stop)) {
      nodes.push(parser());
      if (!this.is(stop)) {
        this.eat(separator);
      }
    }

    this.eat(stop);

    return nodes;
  }
}
