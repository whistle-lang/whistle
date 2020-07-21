import { ParseStringLiteral, StringLiteral } from "./literal.ts";
import { Node, NodeParser } from "./node.ts";
import { WhistleParser } from "./parser.ts";
import { Statement, ParseStatement } from "./statement.ts";
import { ParseTip, Tip } from "./tip.ts";

export interface Program extends Node<ProgramStatement[]> {
  type: "Program";
}

export const ParseProgram: NodeParser<Program> = (parser: WhistleParser) => {
  const program: ProgramStatement[] = [];

  while (parser.current) {
    program.push(ParseProgramStatement(parser));
  }

  return {
    type: "Program",
    value: program,
  };
};

export type ProgramStatement =
  | FunctionDeclaration
  | ImportDeclaration
  | CodeBlock
  | Tip;

export const ParseProgramStatement: NodeParser<ProgramStatement> = (
  parser: WhistleParser,
) => {
  switch (parser.current.type) {
    case "Tip":
      return ParseTip(parser);
    case "Keyword":
      switch (parser.current.value) {
        case "export":
        case "function":
          return ParseFunctionDeclaration(parser);
        case "import":
          return ParseImportDeclaration(parser);
      }
    case "LeftBrace":
      return ParseCodeBlock(parser);
  }

  throw `Could not parse program statement ${JSON.stringify(parser.current)}`;
};

export interface Parameter extends
  Node<{
    name: string;
    type: string;
  }> {
  type: "Parameter";
}

export const ParseParameter: NodeParser<Parameter> = (
  parser: WhistleParser,
) => {
  const name = parser.eat({ type: "Identifier" }).value;

  parser.eat({ type: "Colon" });

  const type = parser.eat({ type: "Type" }).value;

  return {
    type: "Parameter",
    value: { name, type },
  };
};

export interface FunctionDeclaration extends
  Node<{
    exported: boolean;
    name: string;
    parameters: Parameter[];
    type: string;
    body: Statement;
  }> {
  type: "FunctionDeclaration";
}

export const ParseFunctionDeclaration: NodeParser<FunctionDeclaration> = (
  parser: WhistleParser,
) => {
  const exported = parser.is({ type: "Keyword", value: "export" })
    ? parser.eat({ type: "Keyword", value: "export" }) && true
    : false;

  parser.eat({ type: "Keyword", value: "function" });

  const name = parser.eat({ type: "Identifier" }).value;

  let parameters: Parameter[] = [];

  if (parser.is({ type: "LeftParenthesis", value: "(" })) {
    parameters = parser.delimited(
      { type: "LeftParenthesis", value: "(" },
      { type: "RightParenthesis", value: ")" },
      { type: "Comma", value: "," },
      () => ParseParameter(parser),
    );
  }

  parser.eat({ type: "Colon" });

  const type = parser.eat({ type: "Type" }).value;

  const body = ParseStatement(parser);

  return {
    type: "FunctionDeclaration",
    value: {
      exported,
      name,
      parameters,
      type,
      body,
    },
  };
};

export interface ImportDeclaration extends
  Node<{
    names?: string[];
    module: StringLiteral;
  }> {
  type: "ImportDeclaration";
}

export const ParseImportDeclaration: NodeParser<ImportDeclaration> = (
  parser: WhistleParser,
) => {
  parser.eat({ type: "Keyword", value: "import" });

  const names = parser.is({ type: "Identifier" })
    ? parser.until(
      { type: "Keyword", value: "from" },
      { type: "Comma", value: "," },
      (): string => parser.eat({ type: "Identifier" }).value,
    )
    : undefined;

  const module = ParseStringLiteral(parser);

  return {
    type: "ImportDeclaration",
    value: {
      names,
      module,
    },
  };
};

export interface CodeBlock extends Node<Statement[]> {
  type: "CodeBlock";
}

export const ParseCodeBlock: NodeParser<CodeBlock> = (
  parser: WhistleParser,
) => {
  const statements: Statement[] = [];

  parser.eat({ type: "LeftBrace" });

  while (!parser.is({ type: "RightBrace" })) {
    statements.push(ParseStatement(parser));
  }

  parser.eat({ type: "RightBrace" });

  return {
    type: "CodeBlock",
    value: statements,
  };
};
