import { WhistleCompiler } from "../compiler.ts";
import { Operator } from "../../parser/operator.ts";
import { Expression } from "../../parser/expression.ts";
import { Statement } from "../../parser/statement.ts";
import { Program } from "../../parser/program.ts";
import { Context } from "../context.ts";

import { resolve, dirname } from "https://deno.land/std@0.60.0/path/mod.ts";

export const WhistleCompilerJs: WhistleCompiler = (
  program: Program,
  context: Context,
) => {
  let output = "";

  for (const statement of program.value) {
    switch (statement.type) {
      case "FunctionDeclaration":
        if (statement.value.exported) {
          output += "export ";
        }

        output += `function ${statement.value.name}(${
          statement.value
            .parameters.map((parameter) => parameter.value.name).join(
              ",",
            )
        }){${CompileStatement(statement.value.body)}}`;
        break;
      case "ImportDeclaration":
        const module = statement.value.module.value;

        if (statement.value.external) {
          if (statement.value.names) {
            if (module.endsWith(".js")) {
              output += `import{${
                statement.value.names.join(
                  ",",
                )
              }}from"${module}";`;
            }
          } else {
            throw "External imports can must specify what identifiers to import";
          }
        } else {
          if (statement.value.names) {
            throw "Only external imports can import specific identifiers";
          } else {
            const path = resolve(context.directory, module);
            const directory = dirname(path);
            const source = Deno.readTextFileSync(path);

            if (module.endsWith(".js")) {
              output += source;
            }
            if (module.endsWith(".whi")) {
              output += WhistleCompilerJs(source, {
                ...context,
                directory,
              });
            }
          }
        }
        break;
      case "CodeBlock":
        output += `(() => {${
          statement.value.map(CompileStatement).join(
            "",
          )
        }})();`;
        break;
    }
  }

  return output;
};

const CompileOperator = (operator: Operator) => operator.value;

const CompileExpression = (expression: Expression): string => {
  switch (expression.type) {
    case "UnaryExpression":
      return `${CompileOperator(expression.value.operator)}${
        CompileExpression(
          expression.value.operand,
        )
      }`;
    case "BinaryExpression":
      return `${
        CompileExpression(
          expression.value.operandLeft,
        )
      }${
        CompileOperator(
          expression.value.operator,
        )
      }${
        CompileExpression(
          expression.value.operandRight,
        )
      }`;
    case "IfExpression":
      return `${
        CompileExpression(
          expression.value.condition,
        )
      }?${CompileExpression(expression.value.then)}:${
        CompileExpression(
          expression.value.else,
        )
      }`;
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
      return `${expression.value.name}(${
        expression.value.parameters.map(
          CompileExpression,
        ).join(",")
      })`;
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
      return `if(${
        CompileExpression(
          statement.value.condition,
        )
      }){${
        CompileStatement(
          statement.value.then,
        )
      }}${
        statement.value.else
          ? `else{${
            CompileStatement(
              statement.value.else,
            )
          }}`
          : ""
      }`;
    case "LoopStatement":
      return `while(true){${CompileStatement(statement.value)}}`;
    case "WhileStatement":
      return `while(${
        CompileExpression(
          statement.value.condition,
        )
      }){${CompileStatement(statement.value.then)}}`;
    case "ReturnStatement":
      return `return ${CompileExpression(statement.value)};`;
    case "ContinueStatement":
      return `continue;`;
    case "BreakStatement":
      return `break;`;
    case "VariableDeclaration":
      return `let ${statement.value.name}=${
        CompileExpression(
          statement.value.value,
        )
      };`;
    case "ValueDeclaration":
      return `const ${statement.value.name}=${
        CompileExpression(
          statement.value.value,
        )
      };`;
    case "BlockStatement":
      return statement.value.map(CompileStatement).join(
        "",
      );
    case "ExpressionStatement":
      return `${CompileExpression(statement.value)};`;
    default:
      throw `Could not compile statement "${JSON.stringify(statement)}"`;
  }
};
