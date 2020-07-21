import { WhistleParser } from "./parser.ts";
import { Node, NodeParser } from "./node.ts";

export type Literal =
  | IntegerLiteral
  | FloatLiteral
  | CharacterLiteral
  | StringLiteral
  | BooleanLiteral
  | NoneLiteral;

export interface IntegerLiteral extends Node<number> {
  type: "IntegerLiteral";
}

export const ParseIntegerLiteral: NodeParser<IntegerLiteral> = (parser:
  WhistleParser) => {
  return {
    type: "IntegerLiteral",
    value: parser.eat({ type: "Integer" }).value,
  };
};

export interface FloatLiteral extends Node<number> {
  type: "FloatLiteral";
}

export const ParseFloatLiteral: NodeParser<FloatLiteral> = (parser:
  WhistleParser) => {
  return {
    type: "FloatLiteral",
    value: parser.eat({ type: "Float" }).value,
  };
};

export interface CharacterLiteral extends Node<string> {
  type: "CharacterLiteral";
}

export const ParseCharacterLiteral: NodeParser<CharacterLiteral> = (parser:
  WhistleParser) => {
  return {
    type: "CharacterLiteral",
    value: parser.eat({ type: "Character" }).value,
  };
};

export interface StringLiteral extends Node<string> {
  type: "StringLiteral";
}

export const ParseStringLiteral: NodeParser<StringLiteral> = (parser:
  WhistleParser) => {
  return {
    type: "StringLiteral",
    value: parser.eat({ type: "String" }).value,
  };
};

export interface BooleanLiteral extends Node<boolean> {
  type: "BooleanLiteral";
}

export const ParseBooleanLiteral: NodeParser<BooleanLiteral> = (parser:
  WhistleParser) => {
  return {
    type: "BooleanLiteral",
    value: parser.eat({ type: "Boolean" }).value,
  };
};

export interface NoneLiteral extends Node<undefined> {
  type: "NoneLiteral";
}

export const ParseNoneLiteral: NodeParser<NoneLiteral> = (parser:
  WhistleParser) => {
  parser.eat({ type: "Type", value: "none" });

  return {
    type: "NoneLiteral",
    value: undefined,
  };
};
