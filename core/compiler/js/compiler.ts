/* import { WhistleCompiler } from "../compiler.ts";
import {
  Statement,
} from "../../parser/statement.ts";
import {
  Expression,
} from "../../parser/expression.ts";
import { Operator } from "../../parser/operator.ts";

export class WhistleCompilerJs extends WhistleCompiler {
  private static compileOperator<T extends string>(
    operator: Operator<T>,
  ): string {
    return operator.value;
  }

  private static compileExpression<T>(expression: Expression<T>): string {
    let output = "";

    if (expression instanceof UnaryExpression) {
      output += `${WhistleCompilerJs.compileOperator(
        expression.value.operator,
      )}${WhistleCompilerJs.compileExpression(
        expression.value.operand,
      )}`;
    }

    if (expression instanceof BinaryExpression) {
      output += `${WhistleCompilerJs.compileExpression(
        expression.value.operandLeft,
      )}${WhistleCompilerJs.compileOperator(
        expression.value.operator,
      )}${WhistleCompilerJs
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
        WhistleCompilerJs.compileExpression,
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
      output += `if(${WhistleCompilerJs.compileExpression(
        statement.value.condition,
      )}){${WhistleCompilerJs.compileStatement(statement.value.then)}}`;

      if (statement.value.else) {
        output += `else{${WhistleCompilerJs.compileStatement(
          statement.value.else,
        )}}`;
      }
    }

    if (statement instanceof ReturnStatement) {
      output += "return ";
      output += WhistleCompilerJs.compileExpression(statement.value);
      output += ";";
    }

    if (statement instanceof VariableDeclaration) {
      output += `let ${statement.value.name}=${WhistleCompilerJs
        .compileExpression(
          statement.value.value,
        )};`;
    }

    if (statement instanceof BlockStatement) {
      output += statement.value.map(WhistleCompilerJs.compileStatement).join(
        "",
      );
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
          )}){${WhistleCompilerJs.compileStatement(statement.value.body)}}`;
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
*/