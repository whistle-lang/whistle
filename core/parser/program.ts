import { ParseStringLiteral, StringLiteral } from "./literal.ts";
import { Node, NodeParser } from "./node.ts";
import { WhistleParser } from "./parser.ts";
import { Statement, ParseStatement } from "./statement.ts";

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
  | CodeBlock;

export const ParseProgramStatement: NodeParser<ProgramStatement> = (
  parser: WhistleParser,
) => {
  switch (parser.current.type) {
    case "keyword":
      switch (parser.current.value) {
        case "export":
        case "function":
          return ParseFunctionDeclaration(parser);
        case "import":
          return ParseImportDeclaration(parser);
      }
    case "leftBrace":
      return ParseCodeBlock(parser);
  }

  throw `Could not parse program statement ${JSON.stringify(parser.current)}`;
};

export interface Parameter extends Node<{
  name: string;
  type: string;
}> {
  type: "Parameter";
}

export const ParseParameter: NodeParser<Parameter> = (
  parser: WhistleParser,
) => {
  const name = parser.eat({ type: "identifier" }).value;

  parser.eat({ type: "colon" });

  const type = parser.eat({ type: "type" }).value;

  return {
    type: "Parameter",
    value: { name, type },
  };
};

export interface FunctionDeclaration extends Node<{
  exported: boolean;
  name: string;
  parameters: Parameter[];
  type: string;
  body: Statement;
}> {
  type: "FunctionDeclaration";
}

export const ParseFunctionDeclaration: NodeParser<FunctionDeclaration> =
  (parser: WhistleParser) => {
    const exported = parser.is({ type: "keyword", value: "export" })
      ? parser.eat({ type: "keyword", value: "export" }) && true
      : false;

    parser.eat({ type: "keyword", value: "function" });

    const name = parser.eat({ type: "identifier" }).value;

    let parameters: Parameter[] = [];

    if (parser.is({ type: "leftParenthesis", value: "(" })) {
      parameters = parser.delimited(
        { type: "leftParenthesis", value: "(" },
        { type: "rightParenthesis", value: ")" },
        { type: "comma", value: "," },
        () => ParseParameter(parser),
      );
    }

    parser.eat({ type: "colon" });

    const type = parser.eat({ type: "type" }).value;

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

export interface ImportDeclaration extends Node<{
  names: string[];
  module: StringLiteral;
}> {
  type: "ImportDeclaration";
}

export const ParseImportDeclaration: NodeParser<ImportDeclaration> = (
  parser: WhistleParser,
) => {
  return {
    type: "ImportDeclaration",
    value: {
      names: parser.delimited(
        { type: "keyword", value: "import" },
        { type: "keyword", value: "from" },
        { type: "comma", value: "," },
        (): string => parser.eat({ type: "identifier" }).value,
      ),
      module: ParseStringLiteral(parser),
    },
  };
};

export interface CodeBlock extends Node<Statement[]> {
  type: "CodeBlock";
}

export const ParseCodeBlock: NodeParser<CodeBlock> = (parser:
  WhistleParser) => {
  const statements: Statement[] = [];

  parser.eat({ type: "leftBrace" });

  while (!parser.is({ type: "rightBrace" })) {
    statements.push(ParseStatement(parser));
  }

  parser.eat({ type: "rightBrace" });

  return {
    type: "CodeBlock",
    value: statements,
  };
};
