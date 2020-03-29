import { Node } from "./node.ts";
import { WhistleParser } from "./parser.ts";
import {
  BooleanLiteral,
  Integer64Literal,
  Float64Literal,
  CharacterLiteral,
  StringLiteral,
  NoneLiteral
} from "./literal.ts";

export class Expression<T> extends Node<T> {
  public static parse(parser: WhistleParser) {
    switch (parser.current.type) {
      case "boolean":
        return BooleanLiteral.parse(parser);
      case "integer":
        return Integer64Literal.parse(parser);
      case "float":
        return Float64Literal.parse(parser);
      case "character":
        return CharacterLiteral.parse(parser);
      case "string":
        return StringLiteral.parse(parser);
      case "type":
        if (parser.is({ type: "type", value: "none" })) {
          return NoneLiteral.parse(parser);
        }
      case "identifier":
        if (parser.is({ type: "leftParenthesis" }, parser.next)) {
          return FunctionCall.parse(parser);
        } else if (parser.is({ type: "operator", value: "=" }, parser.next)) {
          return VariableAssignment.parse(parser);
        } else {
          return VariableAccess.parse(parser);
        }
    }

    throw `Could not parse expression ${JSON.stringify(parser.current)}`;
  }
}

export class FunctionCall extends Expression<{
  name: string;
  parameters: Expression<any>[];
}> {
  public static parse(parser: WhistleParser): FunctionCall {
    return new FunctionCall({
      name: parser.eat({ type: "identifier" }).value,
      parameters: parser.delimited(
        { type: "leftParenthesis", value: "(" },
        { type: "rightParenthesis", value: ")" },
        { type: "comma", value: "," },
        () => Expression.parse(parser),
      ),
    });
  }
}

export class VariableAssignment extends Expression<{
  name: string;
  value: Expression<any>;
}> {
  public static parse(parser: WhistleParser): VariableAssignment {
    const name = parser.eat({ type: "identifier" }).value;

    parser.eat({ type: "operator", value: "=" });

    const value = Expression.parse(parser);

    return new VariableAssignment({
      name,
      value,
    });
  }
}

export class VariableAccess extends Expression<{
  name: string;
}> {
  public static parse(parser: WhistleParser): VariableAccess {
    return new VariableAccess({
      name: parser.eat({ type: "identifier" }).value,
    });
  }
}
