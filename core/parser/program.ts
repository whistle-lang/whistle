import { StringLiteral } from "./literal.ts";
import { Node, SerializedNode } from "./node.ts";
import { WhistleParser } from "./parser.ts";
import { Statement } from "./statement.ts";

export class Program extends Array<ProgramStatement<any>> {
  public serialize(): SerializedNode<SerializedNode<ProgramStatement<any>>[]> {
    return {
      type: "Program",
      value: this.map((p) => p.serialize()),
    };
  }
}

export class ProgramStatement<T> extends Node<T> {
  public static parse(parser: WhistleParser) {
    switch (parser.current.type) {
      case "keyword":
        switch (parser.current.value) {
          case "export":
          case "function":
            return FunctionDeclaration.parse(parser);
          case "import":
            return ImportDeclaration.parse(parser);
        }
    }

    throw `Could not parse program statement ${JSON.stringify(parser.current)}`;
  }
}

export class Parameter extends Node<{
  name: string;
  type: string;
}> {
  public static parse(parser: WhistleParser) {
    const name = parser.eat({ type: "identifier" }).value;

    parser.eat({ type: "colon" });

    const type = parser.eat({ type: "type" }).value;
    return new Parameter({ name, type });
  }
}

export class FunctionDeclaration extends ProgramStatement<{
  exported: boolean;
  name: string;
  parameters: Parameter[];
  type: string;
  body: Statement<any>;
}> {
  public static parse(parser: WhistleParser): FunctionDeclaration {
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
        () => Parameter.parse(parser),
      );
    }

    parser.eat({ type: "colon" });

    const type = parser.eat({ type: "type" }).value;

    const body = Statement.parse(parser);

    return new FunctionDeclaration({
      exported,
      name,
      parameters,
      type,
      body,
    });
  }
}

export class ImportDeclaration extends ProgramStatement<{
  names: string[];
  module: StringLiteral;
}> {
  public static parse(parser: WhistleParser) {
    return new ImportDeclaration({
      names: parser.delimited(
        { type: "keyword", value: "import" },
        { type: "keyword", value: "from" },
        { type: "comma", value: "," },
        (): string => parser.eat({ type: "identifier" }).value,
      ),
      module: StringLiteral.parse(parser),
    });
  }
}
