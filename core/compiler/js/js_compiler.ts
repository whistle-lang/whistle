import { Compiler } from "../compiler.ts";
import {
  FunctionDeclaration,
  ImportDeclaration
} from "../../parser/program.ts";
import {
  Statement,
  IfStatement,
  ReturnStatement,
  VariableDeclaration,
  BlockStatement
} from "../../parser/statement.ts";
import {
  Expression,
  UnaryExpression,
  BinaryExpression,
  FunctionCall,
  VariableAccess,
  Grouping
} from "../../parser/expression.ts";
import {
  BooleanLiteral,
  Integer32Literal,
  Integer64Literal,
  Float32Literal,
  Float64Literal,
  CharacterLiteral,
  StringLiteral,
  NoneLiteral
} from "../../parser/literal.ts";
import { Operator, UnaryOperator } from "../../parser/operator.ts";

export class JsCompiler extends Compiler {
  private static compileOperator<T extends string>(
    operator: Operator<T>,
  ): string {
    return operator.value;
  }

  private static compileExpression<T>(expression: Expression<T>): string {
    let output = "";

    if (expression instanceof UnaryExpression) {
      output += `${JsCompiler.compileOperator(
        expression.value.operator,
      )}${JsCompiler.compileExpression(
        expression.value.operand,
      )}`;
    }

    if (expression instanceof BinaryExpression) {
      output += `${JsCompiler.compileExpression(
        expression.value.operandLeft,
      )}${JsCompiler.compileOperator(expression.value.operator)}${JsCompiler
        .compileExpression(
          expression.value.operandRight,
        )}`;
    }

    if (
      expression instanceof BooleanLiteral ||
      expression instanceof Integer32Literal ||
      expression instanceof Integer64Literal ||
      expression instanceof Float32Literal ||
      expression instanceof Float64Literal
    ) {
      output += `${expression.value}`;
    }

    if (
      expression instanceof CharacterLiteral ||
      expression instanceof StringLiteral
    ) {
      output += `"${expression.value}"`;
    }

    if (expression instanceof NoneLiteral) {
      output += "undefined";
    }

    if (expression instanceof FunctionCall) {
      output += `${expression.value.name}(${expression.value.parameters.map(
        JsCompiler.compileExpression,
      ).join(",")})`;
    }

    if (expression instanceof VariableAccess) {
      output += `${expression.value.name}`;
    }

    if (expression instanceof Grouping) {
      output += `(${expression.value})`;
    }

    return output;
  }

  private static compileStatement<T>(statement: Statement<T>): string {
    let output = "";

    if (statement instanceof IfStatement) {
      output += `if(${JsCompiler.compileExpression(
        statement.value.condition,
      )}){${JsCompiler.compileStatement(statement.value.then)}}`;

      if (statement.value.else) {
        output += `else{${JsCompiler.compileStatement(
          statement.value.else,
        )}}`;
      }
    }

    if (statement instanceof ReturnStatement) {
      output += "return ";
      output += JsCompiler.compileExpression(statement.value);
      output += ";";
    }

    if (statement instanceof VariableDeclaration) {
      output += `let ${statement.value.name}=${JsCompiler.compileExpression(
        statement.value.value,
      )};`;
    }

    if (statement instanceof BlockStatement) {
      output += statement.value.map(JsCompiler.compileStatement).join("");
    }

    return output;
  }

  public compile(): string {
    let output = "";

    for (const statement of this.program) {
      if (statement instanceof FunctionDeclaration) {
        if (statement.value.exported) {
          output += "export ";
        }

        output += `function ${statement.value.name}(${statement.value
          .parameters.map((parameter) => parameter.value.name).join(
            ",",
          )}){${JsCompiler.compileStatement(statement.value.body)}}`;
      }

      if (statement instanceof ImportDeclaration) {
        output += `import{${statement.value.names.join(
          ",",
        )}}from"${statement.value.module.value}";`;
      }
    }

    return output;
  }
}
