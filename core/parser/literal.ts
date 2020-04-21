import { WhistleParser } from "./parser.ts";
import { Node } from "./node.ts";

export abstract class Literal<T> extends Node<T> {}

export class Integer32Literal extends Literal<number> {
  public static parse(parser: WhistleParser) {
    return new Integer32Literal(parser.eat({ type: "integer" }).value);
  }
}

export class Integer64Literal extends Literal<number> {
  public static parse(parser: WhistleParser) {
    return new Integer64Literal(parser.eat({ type: "integer" }).value);
  }
}

export class Float32Literal extends Literal<number> {
  public static parse(parser: WhistleParser) {
    return new Float32Literal(parser.eat({ type: "float" }).value);
  }
}

export class Float64Literal extends Literal<number> {
  public static parse(parser: WhistleParser) {
    return new Float64Literal(parser.eat({ type: "float" }).value);
  }
}

export class CharacterLiteral extends Literal<string> {
  public static parse(parser: WhistleParser) {
    return new CharacterLiteral(parser.eat({ type: "character" }).value);
  }
}

export class StringLiteral extends Literal<string> {
  public static parse(parser: WhistleParser) {
    return new StringLiteral(parser.eat({ type: "string" }).value);
  }
}

export class BooleanLiteral extends Literal<boolean> {
  public static parse(parser: WhistleParser) {
    return new BooleanLiteral(parser.eat({ type: "boolean" }).value);
  }
}

export class NoneLiteral extends Literal<undefined> {
  public static parse(parser: WhistleParser) {
    parser.eat({ type: "type", value: "none" });

    return new NoneLiteral(undefined);
  }
}
