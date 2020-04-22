import { Node, NodeParser } from "./node.ts";
import { WhistleParser } from "./parser.ts";
import { Expression, ParseExpression } from "./expression.ts";

export type Statement =
  | IfStatement
  | ReturnStatement
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
