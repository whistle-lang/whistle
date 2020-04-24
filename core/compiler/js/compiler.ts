import { WhistleCompiler } from "../compiler.ts";
import { Operator } from "../../parser/operator.ts";
import { Expression } from "../../parser/expression.ts";
import { Statement } from "../../parser/statement.ts";
import { Program } from "../../parser/program.ts";

export const WhistleCompilerJs: WhistleCompiler = (program: Program) => {
  let output = "";

  for (const statement of program.value) {
    switch (statement.type) {
      case "FunctionDeclaration":
        if (statement.value.exported) {
          output += "export ";
        }

        output += `function ${statement.value.name}(${statement.value
          .parameters.map((parameter) => parameter.value.name).join(
            ",",
          )}){${CompileStatement(statement.value.body)}}`;
        break;
      case "ImportDeclaration":
        output += `import{${statement.value.names.join(
          ",",
        )}}from"${statement.value.module.value}";`;
        break;
    }
  }

  return output;
};

const CompileOperator = (operator: Operator) => operator.value;

const CompileExpression = (expression: Expression): string => {
  switch (expression.type) {
    case "UnaryExpression":
      return `${CompileOperator(expression.value.operator)}${CompileExpression(
        expression.value.operand,
      )}`;
    case "BinaryExpression":
      return `${CompileExpression(
        expression.value.operandLeft,
      )}${CompileOperator(
        expression.value.operator,
      )}${CompileExpression(
        expression.value.operandRight,
      )}`;
    case "BooleanLiteral":
    case "IntegerLiteral":
    case "FloatLiteral":
      return `${expression.value}`;
    case "CharacterLiteral":
    case "StringLiteral":
      return `"${expression.value}"`;
    case "NoneLiteral":
      return "undefined";
    case "FunctionCall":
      return `${expression.value.name}(${expression.value.parameters.map(
        CompileExpression,
      ).join(",")})`;
    case "VariableAccess":
      return `${expression.value.name}`;
    case "Grouping":
      return `(${expression.value})`;
    default:
      throw `Could not compile expression "${JSON.stringify(expression)}"`;
  }
};

const CompileStatement = (statement: Statement): string => {
  switch (statement.type) {
    case "IfStatement":
      return `if(${CompileExpression(
        statement.value.condition,
      )}){${CompileStatement(
        statement.value.then,
      )}}${statement.value.else
        ? `else{${CompileStatement(
          statement.value.else,
        )}}`
        : ""}`;
    case "ReturnStatement":
      return `return ${CompileExpression(statement.value)};`;
    case "VariableDeclaration":
      return `let ${statement.value.name}=${CompileExpression(
        statement.value.value,
      )};`;
    case "BlockStatement":
      return statement.value.map(CompileStatement).join(
        "",
      );
    case "ExpressionStatement":
      return `(${CompileExpression(statement.value)});`;
    default:
      throw `Could not compile statement "${JSON.stringify(statement)}"`;
  }
};
