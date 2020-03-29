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
import { UnaryOperator, BinaryOperator } from "./operator.ts";

export class Expression<T> extends Node<T> {
  public static parse(parser: WhistleParser, ...params: any[]) {
    return Expression.parseWithPrecedence(parser, -1);
  }

  public static parseWithPrecedence(parser: WhistleParser, precedence: number) {
    let left = Expression.parsePrimaryOrUnaryExpression(parser);

    while (
      BinaryOperator.is(parser.current) &&
      BinaryOperator.from(parser.current).precedence > precedence
    ) {
      left = BinaryExpression.parse(parser, left);
      console.log(left);
      
    }

    return left;
  }

  public static parsePrimaryOrUnaryExpression(
    parser: WhistleParser,
  ): PrimaryExpression<any> | UnaryExpression {
    if (UnaryOperator.is(parser.current)) {
      return UnaryExpression.parse(parser);
    } else {
      return PrimaryExpression.parse(parser);
    }
  }
}

export class UnaryExpression extends Expression<{
  operator: UnaryOperator;
  operand: Expression<any>;
}> {
  public static parse(parser: WhistleParser): UnaryExpression {
    return new UnaryExpression({
      operator: UnaryOperator.parse(parser),
      operand: Expression.parsePrimaryOrUnaryExpression(parser),
    });
  }
}

export class BinaryExpression extends Expression<{
  operandLeft: Expression<any>;
  operator: BinaryOperator<any>;
  operandRight: Expression<any>;
}> {
  public static parse(
    parser: WhistleParser,
    left: Expression<any>,
  ): BinaryExpression {
    const operator = BinaryOperator.parse(parser);
    return new BinaryExpression({
      operandLeft: left,
      operator,
      operandRight: Expression.parseWithPrecedence(parser, operator.precedence),
    });
  }
}

export class PrimaryExpression<T> extends Expression<T> {
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
        } else {
          return VariableAccess.parse(parser);
        }
      case "leftParenthesis":
        return Grouping.parse(parser);
    }

    throw `Could not parse primary expression ${JSON.stringify(
      parser.current,
    )}`;
  }
}

export class FunctionCall extends PrimaryExpression<{
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

export class VariableAccess extends PrimaryExpression<{
  name: string;
}> {
  public static parse(parser: WhistleParser): VariableAccess {
    return new VariableAccess({
      name: parser.eat({ type: "identifier" }).value,
    });
  }
}

export class Grouping extends PrimaryExpression<Expression<any>> {
  public static parse(parser: WhistleParser): Grouping {
    return new Grouping(Expression.parse(parser));
  }
}
