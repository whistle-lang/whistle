import {
  CompilationTarget,
  CompilationSource,
} from "../types.ts";
import { Operator } from "../../parser/operator.ts";
import { Expression } from "../../parser/expression.ts";
import { Statement } from "../../parser/statement.ts";
import {
  Program,
  Parameter,
  CodeBlock,
  FunctionDeclaration,
} from "../../parser/program.ts";
import { Tip } from "../../parser/tip.ts";

export class JsCompilationTarget extends CompilationTarget<string> {
  compile(source: CompilationSource): string {
    let result = "";

    for (const external of source.external) {
      result += typeof external === "string" ? external : JsCompilationTarget.CompileFunctionDeclaration(external, true);
    }

    result += JsCompilationTarget.CompileProgram(source.program);

    return result;
  }

  private static CompileProgram(program: Program): string {
    for (const statement of program.value) {
      switch (statement.type) {
        case "Tip":
          return JsCompilationTarget.CompileTip(statement);
        case "CodeBlock":
          return JsCompilationTarget.CompileCodeBlock(statement);
        case "FunctionDeclaration":
          return JsCompilationTarget.CompileFunctionDeclaration(statement, false);
      }
    }
    
    return "";
  }

  private static CompileCodeBlock(block: CodeBlock): string {
    return `(()=>{${
      block.value.map(JsCompilationTarget.CompileStatement).join(
        "",
      )
    }})();`;
  }

  private static CompileFunctionDeclaration(declaration: FunctionDeclaration, external: boolean): string {
    let result = "";

    if (declaration.value.exported && !external) {
      result += "export ";
    }

    result += `function ${declaration.value.name}(${
      declaration.value
        .parameters.map((parameter: Parameter) => parameter.value.name).join(
          ",",
        )
    }){${JsCompilationTarget.CompileStatement(declaration.value.body)}}`;

    return result;
  }

  private static CompileTip(tip: Tip): string {
    return tip.value.type === "js" ? tip.value.value : "";
  }

  private static CompileStatement(statement: Statement): string {
    switch (statement.type) {
      case "Tip":
          return JsCompilationTarget.CompileTip(statement);
      case "IfStatement":
        return `if(${
          JsCompilationTarget.CompileExpression(
            statement.value.condition,
          )
        }){${
          JsCompilationTarget.CompileStatement(
            statement.value.then,
          )
        }}${
          statement.value.else
            ? `else{${
              JsCompilationTarget.CompileStatement(
                statement.value.else,
              )
            }}`
            : ""
        }`;
      case "LoopStatement":
        return `while(true){${JsCompilationTarget.CompileStatement(statement.value)}}`;
      case "WhileStatement":
        return `while(${
          JsCompilationTarget.CompileExpression(
            statement.value.condition,
          )
        }){${JsCompilationTarget.CompileStatement(statement.value.then)}}`;
      case "ReturnStatement":
        return `return ${JsCompilationTarget.CompileExpression(statement.value)};`;
      case "ContinueStatement":
        return `continue;`;
      case "BreakStatement":
        return `break;`;
      case "VariableDeclaration":
        return `let ${statement.value.name}=${
          JsCompilationTarget.CompileExpression(
            statement.value.value,
          )
        };`;
      case "ValueDeclaration":
        return `const ${statement.value.name}=${
          JsCompilationTarget.CompileExpression(
            statement.value.value,
          )
        };`;
      case "BlockStatement":
        return statement.value.map(JsCompilationTarget.CompileStatement).join(
          "",
        );
      case "ExpressionStatement":
        return `${JsCompilationTarget.CompileExpression(statement.value)};`;
      default:
        throw `Could not compile statement "${JSON.stringify(statement)}"`;
    }
  }

  private static CompileExpression(expression: Expression): string {
    switch (expression.type) {
      case "UnaryExpression":
        return `${JsCompilationTarget.CompileOperator(expression.value.operator)}${
          JsCompilationTarget.CompileExpression(
            expression.value.operand,
          )
        }`;
      case "BinaryExpression":
        return `${
          JsCompilationTarget.CompileExpression(
            expression.value.operandLeft,
          )
        }${
          JsCompilationTarget.CompileOperator(
            expression.value.operator,
          )
        }${
          JsCompilationTarget.CompileExpression(
            expression.value.operandRight,
          )
        }`;
      case "IfExpression":
        return `${
          JsCompilationTarget.CompileExpression(
            expression.value.condition,
          )
        }?${JsCompilationTarget.CompileExpression(expression.value.then)}:${
          JsCompilationTarget.CompileExpression(
            expression.value.else,
          )
        }`;
      case "BooleanLiteral":
      case "IntegerLiteral":
      case "FloatLiteral":
        return expression.value.toString();
      case "CharacterLiteral":
      case "StringLiteral":
        return `"${expression.value}"`;
      case "NoneLiteral":
        return "undefined";
      case "FunctionCall":
        return `${expression.value.name}(${
          expression.value.parameters.map(JsCompilationTarget.CompileExpression)
            .join(",")
        })`;
      case "VariableAccess":
        return `${expression.value.name}`;
      case "Grouping":
        return `(${expression.value})`;
      default:
        throw `Could not compile expression "${JSON.stringify(expression)}"`;
    }
  }

  private static CompileOperator(operator: Operator): string {
    return operator.value;
  }
}
