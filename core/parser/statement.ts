import { Node, NodeParser } from "./node.ts";
import { WhistleParser } from "./parser.ts";
import { Expression, ParseExpression } from "./expression.ts";

export type Statement =
  | IfStatement
  | LoopStatement
  | WhileStatement
  | ReturnStatement
  | ContinueStatement
  | BreakStatement
  | VariableDeclaration
  | BlockStatement
  | ExpressionStatement;

export const ParseStatement: NodeParser<Statement> = (parser:
  WhistleParser) => {
  switch (parser.current.type) {
    case "keyword":
      switch (parser.current.value) {
        case "if":
          return ParseIfStatement(parser);
        case "return":
          return ParseReturnStatement(parser);
        case "var":
          return ParseVariableDeclaration(parser);
      }
    case "leftBrace":
      return ParseBlockStatement(parser);
    default:
      return ParseExpressionStatement(parser);
  }

  throw `Could not parse statement ${JSON.stringify(parser.current)}`;
};

export interface ReturnStatement extends Node<Expression> {
  type: "ReturnStatement";
}

export const ParseReturnStatement: NodeParser<ReturnStatement> = (parser:
  WhistleParser) => {
  parser.eat({ type: "keyword", value: "return" });

  return {
    type: "ReturnStatement",
    value: ParseExpression(parser),
  };
};

export interface ContinueStatement extends Node<undefined> {
  type: "ContinueStatement";
}

export const ParseContinueStatement: NodeParser<ContinueStatement> = (parser:
  WhistleParser) => {
  parser.eat({ type: "keyword", value: "continue" });

  return {
    type: "ContinueStatement",
    value: undefined,
  };
};

export interface BreakStatement extends Node<undefined> {
  type: "BreakStatement";
}

export const ParseBreakStatement: NodeParser<BreakStatement> = (parser:
  WhistleParser) => {
  parser.eat({ type: "keyword", value: "break" });

  return {
    type: "BreakStatement",
    value: undefined,
  };
};

export interface IfStatement extends Node<{
  condition: Expression;
  then: Statement;
  else?: Statement;
}> {
  type: "IfStatement";
}

export const ParseIfStatement: NodeParser<IfStatement> = (parser:
  WhistleParser) => {
  parser.eat({ type: "keyword", value: "if" });

  return {
    type: "IfStatement",
    value: {
      condition: ParseExpression(parser),
      then: ParseStatement(parser),
      else: parser.is({ type: "keyword", value: "else" })
        ? parser.eat({ type: "keyword", value: "else" }) &&
          ParseStatement(parser)
        : undefined,
    },
  };
};

export interface LoopStatement extends Node<Statement> {
  type: "LoopStatement";
}

export const ParseLoopStatement: NodeParser<LoopStatement> = (parser:
  WhistleParser) => {
  parser.eat({ type: "keyword", value: "loop" });

  return {
    type: "LoopStatement",
    value: ParseStatement(parser),
  };
};

export interface WhileStatement extends Node<{
  condition: Expression;
  then: Statement;
}> {
  type: "WhileStatement";
}

export const ParseWhileStatement: NodeParser<WhileStatement> = (parser:
  WhistleParser) => {
  parser.eat({ type: "keyword", value: "while" });

  return {
    type: "WhileStatement",
    value: {
      condition: ParseExpression(parser),
      then: ParseStatement(parser),
    },
  };
};

export interface VariableDeclaration extends Node<{
  name: string;
  type: string;
  value: Expression;
}> {
  type: "VariableDeclaration";
}

export const ParseVariableDeclaration: NodeParser<VariableDeclaration> = (
  parser: WhistleParser,
) => {
  parser.eat({ type: "keyword", value: "var" });

  const name = parser.eat({ type: "identifier" }).value;

  parser.eat({ type: "colon" });

  const type = parser.eat({ type: "type" }).value;

  parser.eat({ type: "operator", value: "=" });

  const value = ParseExpression(parser);

  return {
    type: "VariableDeclaration",
    value: {
      name,
      type,
      value,
    },
  };
};

export interface BlockStatement extends Node<Statement[]> {
  type: "BlockStatement";
}

export const ParseBlockStatement: NodeParser<BlockStatement> = (parser:
  WhistleParser) => {
  const statements: Statement[] = [];

  parser.eat({ type: "leftBrace" });

  while (!parser.is({ type: "rightBrace" })) {
    statements.push(ParseStatement(parser));
  }

  parser.eat({ type: "rightBrace" });

  return {
    type: "BlockStatement",
    value: statements,
  };
};

export interface ExpressionStatement extends Node<Expression> {
  type: "ExpressionStatement";
}

export const ParseExpressionStatement: NodeParser<ExpressionStatement> =
  (parser: WhistleParser) => {
    return {
      type: "ExpressionStatement",
      value: ParseExpression(parser),
    };
  };
