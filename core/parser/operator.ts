import { Node } from "./node.ts";
import { WhistleParser } from "./parser.ts";
import { Token } from "https://deno.land/x/tokenizer/token.ts";

// TODO: Add binary associativity and unary notation
// export enum Associativity {
//   left,
//   none,
//   right,
// }

// export enum Notation {
//   prefix,
//   postfix,
// }

export abstract class Operator<T extends string> extends Node<T> {
  public static is(token: Token): boolean {
    return false;
  }
}

export class UnaryOperator extends Operator<"+" | "-" | "!"> {
  public static parse(parser: WhistleParser): UnaryOperator {
    return new UnaryOperator(parser.advance().value);
  }

  public static is(token: Token): boolean {
    return token.type === "operator" && ["+", "-", "!"].includes(token.value);
  }
}

export abstract class BinaryOperator<T extends string> extends Operator<T> {
  public abstract readonly precedence: number;

  public static parse(parser: WhistleParser): BinaryOperator<any> {
    return BinaryOperator.from(parser.advance());
  }

  public static is(token: Token): boolean {
    return token.type === "operator" &&
      [
        "=",
        "+",
        "-",
        "*",
        "/",
        "%",
        "==",
        "!=",
        "<",
        ">",
        "<=",
        ">=",
        "||",
        "&&",
      ].includes(token.value);
  }

  public static from(token: Token): BinaryOperator<any> {
    switch (token.value) {
      case "!=":
      case "==":
        return new EqualityOperator(token.value);
      case "<":
      case "<=":
      case ">":
      case ">=":
        return new ComparisonOperator(token.value);
      case "+":
      case "-":
        return new AdditionOperator(token.value);
      case "*":
      case "/":
      case "%":
        return new MultiplicationOperator(token.value);
      case "&&":
      case "||":
        return new BooleanOperator(token.value);
    }

    throw `Unknown binary operator "${token.value}"`;
  }
}

export class EqualityOperator extends BinaryOperator<"!=" | "=="> {
  public readonly precedence = 0;
}

export class ComparisonOperator
  extends BinaryOperator<"<" | "<=" | ">" | ">="> {
  public readonly precedence = 1;
}

export class AdditionOperator extends BinaryOperator<"+" | "-"> {
  public readonly precedence = 2;
}

export class MultiplicationOperator extends BinaryOperator<"*" | "/" | "%"> {
  public readonly precedence = 3;
}

export class BooleanOperator extends BinaryOperator<"&&" | "||"> {
  public readonly precedence = 4;
}
