import { Node, NodeParser } from "./node.ts";
import { WhistleParser } from "./parser.ts";
import {
  Literal,
  ParseBooleanLiteral,
  ParseIntegerLiteral,
  ParseFloatLiteral,
  ParseStringLiteral,
  ParseCharacterLiteral,
  ParseNoneLiteral,
} from "./literal.ts";
import {
  UnaryOperator,
  BinaryOperator,
  IsUnaryOperator,
  ParseUnaryOperator,
  IsBinaryOperator,
  GetBinaryOperator,
  ParseBinaryOperator,
} from "./operator.ts";

export type Expression =
  | UnaryExpression
  | BinaryExpression
  | FunctionCall
  | VariableAccess
  | Grouping
  | Literal;

export const ParseExpression: NodeParser<Expression> = (parser:
  WhistleParser) => {
  return ParseExpressionWithPrecedence(parser, -1);
};

export const ParseExpressionWithPrecedence: NodeParser<Expression> = (
  parser: WhistleParser,
  precedence: number,
) => {
  let left: PrimaryExpression | UnaryExpression | BinaryExpression =
    ParsePrimaryOrUnaryExpression(parser);

  while (
    IsBinaryOperator(parser.current) &&
    GetBinaryOperator(parser.current).precedence > precedence
  ) {
    left = ParseBinaryExpression(parser, left);
  }

  return left;
};

export const ParsePrimaryOrUnaryExpression: NodeParser<
  PrimaryExpression | UnaryExpression
> = (parser: WhistleParser) => {
  if (IsUnaryOperator(parser.current)) {
    return ParseUnaryExpression(parser);
  } else {
    return ParsePrimaryExpression(parser);
  }
};

export interface UnaryExpression extends Node<{
  operator: UnaryOperator;
  operand: Expression;
}> {
  type: "UnaryExpression";
}

export const ParseUnaryExpression: NodeParser<UnaryExpression> = (parser:
  WhistleParser) => {
  return {
    type: "UnaryExpression",
    value: {
      operator: ParseUnaryOperator(parser),
      operand: ParsePrimaryOrUnaryExpression(parser),
    },
  };
};

export interface BinaryExpression extends Node<{
  operandLeft: Expression;
  operator: BinaryOperator;
  operandRight: Expression;
}> {
  type: "BinaryExpression";
}

export const ParseBinaryExpression: NodeParser<BinaryExpression> = (
  parser: WhistleParser,
  left: Expression,
) => {
  const operator = ParseBinaryOperator(parser);

  return {
    type: "BinaryExpression",
    value: {
      operandLeft: left,
      operator,
      operandRight: ParseExpressionWithPrecedence(parser, operator.precedence),
    },
  };
};

export type PrimaryExpression =
  | Literal
  | FunctionCall
  | VariableAccess
  | Grouping;

export const ParsePrimaryExpression: NodeParser<PrimaryExpression> = (parser:
  WhistleParser) => {
  switch (parser.current.type) {
    case "boolean":
      return ParseBooleanLiteral(parser);
    case "integer":
      return ParseIntegerLiteral(parser);
    case "float":
      return ParseFloatLiteral(parser);
    case "character":
      return ParseCharacterLiteral(parser);
    case "string":
      return ParseStringLiteral(parser);
    case "type":
      if (parser.is({ type: "type", value: "none" })) {
        return ParseNoneLiteral(parser);
      }
    case "identifier":
      if (parser.is({ type: "leftParenthesis" }, parser.next)) {
        return ParseFunctionCall(parser);
      } else {
        return ParseVariableAccess(parser);
      }
    case "leftParenthesis":
      return ParseGrouping(parser);
  }

  throw `Could not parse primary expression ${JSON.stringify(
    parser.current,
  )}`;
};

export interface FunctionCall extends Node<{
  name: string;
  parameters: Expression[];
}> {
  type: "FunctionCall";
}

export const ParseFunctionCall: NodeParser<FunctionCall> = (parser:
  WhistleParser) => {
  return {
    type: "FunctionCall",
    value: {
      name: parser.eat({ type: "identifier" }).value,
      parameters: parser.delimited(
        { type: "leftParenthesis", value: "(" },
        { type: "rightParenthesis", value: ")" },
        { type: "comma", value: "," },
        () => ParseExpression(parser),
      ),
    },
  };
};

export interface VariableAccess extends Node<{
  name: string;
}> {
  type: "VariableAccess";
}

export const ParseVariableAccess: NodeParser<VariableAccess> = (parser:
  WhistleParser) => {
  return {
    type: "VariableAccess",
    value: {
      name: parser.eat({ type: "identifier" }).value,
    },
  };
};

export interface Grouping extends Node<Expression> {
  type: "Grouping";
}

export const ParseGrouping: NodeParser<Grouping> = (parser: WhistleParser) => {
  return {
    type: "Grouping",
    value: ParseExpression(parser),
  };
};
