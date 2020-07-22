import { Node, NodeParser } from "./node.ts";
import { WhistleParser } from "./parser.ts";
import { Token } from "../tokenizer/token.ts";

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

export type Operator = UnaryOperator | BinaryOperator;

export type UnaryOperator =
  | PositiveOperator
  | NegativeOperator
  | LogicalNegateOperator;

export const ParseUnaryOperator: NodeParser<UnaryOperator> = (
  parser: WhistleParser,
) => {
  const operator = parser.eat({ type: "Operator" }).value;
  switch (operator) {
    case "+":
      return { type: "PositiveOperator", value: "+" };
    case "-":
      return { type: "NegativeOperator", value: "-" };
    case "!":
      return { type: "LogicalNegateOperator", value: "!" };
    default:
      throw `Unknown unary operator "${operator}"`;
  }
};

export const IsUnaryOperator = (token: Token): boolean => {
  return token.type === "Operator" && ["+", "-", "!"].includes(token.value);
};

export interface PositiveOperator extends Node<"+"> {
  type: "PositiveOperator";
}

export interface NegativeOperator extends Node<"-"> {
  type: "NegativeOperator";
}

export interface LogicalNegateOperator extends Node<"!"> {
  type: "LogicalNegateOperator";
}

export type BinaryOperator =
  | AssignmentOperator
  | AdditionOperator
  | SubtractionOperator
  | MultiplicationOperator
  | DivisionOperator
  | ModuloOperator
  | EqualOperator
  | NotEqualOperator
  | LessThanOperator
  | GreaterThanOperator
  | LessThanOrEqualOperator
  | GreaterThanOrEqualOperator
  | LogicalOrOperator
  | LogicalAndOperator;

export const ParseBinaryOperator: NodeParser<BinaryOperator> = (
  parser: WhistleParser,
) => {
  return GetBinaryOperator(parser.eat({ type: "Operator" }));
};

export const GetBinaryOperator = (token: Token): BinaryOperator => {
  switch (token.value) {
    case "=":
      return { type: "AssignmentOperator", precedence: 5, value: "=" };
    case "+":
      return { type: "AdditionOperator", precedence: 2, value: "+" };
    case "-":
      return { type: "SubtractionOperator", precedence: 2, value: "-" };
    case "*":
      return { type: "MultiplicationOperator", precedence: 3, value: "*" };
    case "/":
      return { type: "DivisionOperator", precedence: 3, value: "/" };
    case "%":
      return { type: "ModuloOperator", precedence: 3, value: "%" };
    case "==":
      return { type: "EqualOperator", precedence: 0, value: "==" };
    case "!=":
      return { type: "NotEqualOperator", precedence: 0, value: "!=" };
    case "<":
      return { type: "LessThanOperator", precedence: 1, value: "<" };
    case ">":
      return { type: "GreaterThanOperator", precedence: 1, value: ">" };
    case "<=":
      return { type: "LessThanOrEqualOperator", precedence: 1, value: "<=" };
    case ">=":
      return { type: "GreaterThanOrEqualOperator", precedence: 1, value: ">=" };
    case "||":
      return { type: "LogicalOrOperator", precedence: 4, value: "||" };
    case "&&":
      return { type: "LogicalAndOperator", precedence: 4, value: "&&" };
    default:
      throw `Unknown binary operator "${token.value}"`;
  }
};

export const IsBinaryOperator = (token: Token): boolean => {
  return token.type === "Operator" &&
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
};

export interface BinaryOperatorNode<T> extends Node<T> {
  precedence: number;
}

export interface AssignmentOperator extends BinaryOperatorNode<"="> {
  type: "AssignmentOperator";
  precedence: 5;
}

export interface AdditionOperator extends BinaryOperatorNode<"+"> {
  type: "AdditionOperator";
  precedence: 2;
}

export interface SubtractionOperator extends BinaryOperatorNode<"-"> {
  type: "SubtractionOperator";
  precedence: 2;
}

export interface MultiplicationOperator extends BinaryOperatorNode<"*"> {
  type: "MultiplicationOperator";
  precedence: 3;
}

export interface DivisionOperator extends BinaryOperatorNode<"/"> {
  type: "DivisionOperator";
  precedence: 3;
}

export interface ModuloOperator extends BinaryOperatorNode<"%"> {
  type: "ModuloOperator";
  precedence: 3;
}

export interface EqualOperator extends BinaryOperatorNode<"=="> {
  type: "EqualOperator";
  precedence: 0;
}

export interface NotEqualOperator extends BinaryOperatorNode<"!="> {
  type: "NotEqualOperator";
  precedence: 0;
}

export interface LessThanOperator extends BinaryOperatorNode<"<"> {
  type: "LessThanOperator";
  precedence: 1;
}

export interface GreaterThanOperator extends BinaryOperatorNode<">"> {
  type: "GreaterThanOperator";
  precedence: 1;
}

export interface LessThanOrEqualOperator extends BinaryOperatorNode<"<="> {
  type: "LessThanOrEqualOperator";
  precedence: 1;
}

export interface GreaterThanOrEqualOperator extends BinaryOperatorNode<">="> {
  type: "GreaterThanOrEqualOperator";
  precedence: 1;
}

export interface LogicalOrOperator extends BinaryOperatorNode<"||"> {
  type: "LogicalOrOperator";
  precedence: 4;
}

export interface LogicalAndOperator extends BinaryOperatorNode<"&&"> {
  type: "LogicalAndOperator";
  precedence: 4;
}
