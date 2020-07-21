import { Node, NodeParser } from "./node.ts";
import { WhistleParser } from "./parser.ts";
import { Expression, ParseExpression } from "./expression.ts";
import { ParseTip, Tip } from "./tip.ts";

export type Statement =
  | IfStatement
  | LoopStatement
  | WhileStatement
  | ReturnStatement
  | ContinueStatement
  | BreakStatement
  | VariableDeclaration
  | ValueDeclaration
  | BlockStatement
  | ExpressionStatement
  | Tip;

export const ParseStatement: NodeParser<Statement> = (parser:
  WhistleParser) => {
  switch (parser.current.type) {
    case "Tip":
      return ParseTip(parser);
    case "Keyword":
      switch (parser.current.value) {
        case "if":
          return ParseIfStatement(parser);
        case "return":
          return ParseReturnStatement(parser);
        case "var":
          return ParseVariableDeclaration(parser);
        case "val":
          return ParseValueDeclaration(parser);
        case "loop":
          return ParseLoopStatement(parser);
        case "while":
          return ParseWhileStatement(parser);
        case "continue":
          return ParseContinueStatement(parser);
        case "break":
          return ParseBreakStatement(parser);
      }
    case "LeftBrace":
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
  parser.eat({ type: "Keyword", value: "return" });

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
  parser.eat({ type: "Keyword", value: "continue" });

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
  parser.eat({ type: "Keyword", value: "break" });

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
  parser.eat({ type: "Keyword", value: "if" });

  return {
    type: "IfStatement",
    value: {
      condition: ParseExpression(parser),
      then: ParseStatement(parser),
      else: parser.is({ type: "Keyword", value: "else" })
        ? parser.eat({ type: "Keyword", value: "else" }) &&
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
  parser.eat({ type: "Keyword", value: "loop" });

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
  parser.eat({ type: "Keyword", value: "while" });

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
  parser.eat({ type: "Keyword", value: "var" });

  const name = parser.eat({ type: "Identifier" }).value;

  parser.eat({ type: "Colon" });

  const type = parser.eat({ type: "Type" }).value;

  parser.eat({ type: "Operator", value: "=" });

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

export interface ValueDeclaration extends Node<{
  name: string;
  type: string;
  value: Expression;
}> {
  type: "ValueDeclaration";
}

export const ParseValueDeclaration: NodeParser<ValueDeclaration> = (
  parser: WhistleParser,
) => {
  parser.eat({ type: "Keyword", value: "val" });

  const name = parser.eat({ type: "Identifier" }).value;

  parser.eat({ type: "Colon" });

  const type = parser.eat({ type: "Type" }).value;

  parser.eat({ type: "Operator", value: "=" });

  const value = ParseExpression(parser);

  return {
    type: "ValueDeclaration",
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

  parser.eat({ type: "LeftBrace" });

  while (!parser.is({ type: "RightBrace" })) {
    statements.push(ParseStatement(parser));
  }

  parser.eat({ type: "RightBrace" });

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
