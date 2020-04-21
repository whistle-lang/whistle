import { Node } from "./node.ts";
import { WhistleParser } from "./parser.ts";
import { Expression } from "./expression.ts";

export class Statement<T> extends Node<T> {
  public static parse(parser: WhistleParser) {
    switch (parser.current.type) {
      case "keyword":
        switch (parser.current.value) {
          case "if":
            return IfStatement.parse(parser);
          case "return":
            return ReturnStatement.parse(parser);
          case "var":
            return VariableDeclaration.parse(parser);
        }
      case "leftBrace":
        return BlockStatement.parse(parser);
    }

    throw `Could not parse statement ${JSON.stringify(parser.current)}`;
  }
}

export class ReturnStatement extends Statement<Expression<any>> {
  public static parse(parser: WhistleParser) {
    parser.eat({ type: "keyword", value: "return" });

    return new ReturnStatement(Expression.parse(parser));
  }
}

export class IfStatement extends Statement<{
  condition: Expression<any>;
  then: Statement<any>;
  else: Statement<any> | undefined;
}> {
  public static parse(parser: WhistleParser): IfStatement {
    parser.eat({ type: "keyword", value: "if" });

    return new IfStatement({
      condition: Expression.parse(parser),
      then: Statement.parse(parser),
      else: parser.is({ type: "keyword", value: "else" })
        ? parser.eat({ type: "keyword", value: "else" }) &&
          Statement.parse(parser)
        : undefined,
    });
  }
}

export class VariableDeclaration extends Statement<{
  name: string;
  type: string;
  value: Expression<any>;
}> {
  public static parse(parser: WhistleParser): VariableDeclaration {
    parser.eat({ type: "keyword", value: "var" });

    const name = parser.eat({ type: "identifier" }).value;

    parser.eat({ type: "colon" });

    const type = parser.eat({ type: "type" }).value;

    parser.eat({ type: "operator", value: "=" });

    const value = Expression.parse(parser);

    return new VariableDeclaration({
      name,
      type,
      value,
    });
  }
}

export class BlockStatement extends Statement<Statement<any>[]> {
  public static parse(parser: WhistleParser): BlockStatement {
    const statements: Statement<any>[] = [];

    parser.eat({ type: "leftBrace" });

    while (!parser.is({ type: "rightBrace" })) {
      statements.push(Statement.parse(parser));
    }

    parser.eat({ type: "rightBrace" });

    return new BlockStatement(statements);
  }
}
