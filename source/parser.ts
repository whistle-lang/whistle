import { Token, Tokenizer } from "https://deno.land/x/tokenizer/mod.ts";

abstract class Node<T> {
    public value: T;

    constructor(value: T) {
        this.value = value;
    }
}

type Expression = LeftUnaryExpression | BinaryExpression | RightUnaryExpression | VariableAccess;

type Statement =
    | VariableDeclaration
    | VariableAssignment
    | FunctionDeclarationStatement
    | FunctionCallStatement
    | IfStatement
    | ImportStatement
    | ReturnStatement
    | BinaryOperatorDeclarationStatement
    | RightUnaryOperatorDeclarationStatement
    | LeftUnaryOperatorDeclarationStatement;

type Program = Statement[];

class BinaryExpression extends Node<{
    left: Expression;
    operator: string;
    right: Expression;
}> {}

class LeftUnaryExpression extends Node<{
    left: Expression;
    operator: string;
}> {}

class RightUnaryExpression extends Node<{
    operator: string;
    right: Expression;
}> {}

class VariableAccess extends Node<{
    name: string;
}> {}

class VariableDeclaration extends Node<{
    name: string;
    type: string;
    value?: Expression;
}> {}

class VariableAssignment extends Node<{
    name: string;
    value: Expression;
}> {}

class ParameterDeclaration extends Node<{
    name: string;
    type: string;
}> {}

class FunctionDeclarationStatement extends Node<{
    exported: boolean;
    name: string;
    args: ParameterDeclaration[];
    type: string;
    statements: Statement[];
}> {}

class FunctionCallStatement extends Node<{
    exported: boolean;
    name: string;
    args: Expression[];
}> {}

class BinaryOperatorDeclarationStatement extends Node<{
    left: ParameterDeclaration;
    symbol: string;
    right: ParameterDeclaration;
    type: string;
    statements: Statement[];
}> {}

class LeftUnaryOperatorDeclarationStatement extends Node<{
    left: ParameterDeclaration;
    symbol: string;
    type: string;
    statements: Statement[];
}> {}

class RightUnaryOperatorDeclarationStatement extends Node<{
    symbol: string;
    right: ParameterDeclaration;
    type: string;
    statements: Statement[];
}> {}

class ReturnStatement extends Node<{
    value: Expression;
}> {}

class IfStatement extends Node<{
    condition: Expression;
    then: Statement[];
    else: Statement[];
}> {}

class ImportStatement extends Node<{
    module: string;
    names: string[];
}> {}

export function parse(tokens: Token[]): Program {
    let index = 0;

    const current = () => tokens[index];
    const next = () => tokens[index + 1];

    function eat({ value, type }: { value?: string; type?: string }) {
        if (!current()) {
            throw `Unexpected end of file`;
        }

        if (type && value) {
            if (current().type !== type && current().value !== value) {
                throw `Expected type "${type}" with value "${value}" but recieved "${
                    current().type
                }" with value "${current().value}" at index ${current().position.start}`;
            }

            if (current().type !== type) {
                throw `Expected type "${type}" but recieved "${current().type}" at index ${
                    current().position.start
                }`;
            }

            if (current().value !== value) {
                throw `Expected value "${type}" but recieved "${current().type}" at index ${
                    current().position.start
                }`;
            }
        } else {
            if (type) {
                if (current().type !== type) {
                    throw `Expected type "${type}" but recieved "${current().type}" at index ${
                        current().position.start
                    }`;
                }
            }
            if (value) {
                if (current().value !== value) {
                    throw `Expected value "${type}" but recieved "${current().type}" at index ${
                        current().position.start
                    }`;
                }
            }
        }

        return tokens[index++];
    }

    function is({ value, type }: { value?: string; type?: string }) {
        if (!current()) {
            throw `Unexpected end of file`;
        }

        if (type && value) {
            return current().type === type && current().value === value;
        } else {
            if (type) {
                return current().type === type;
            }

            if (value) {
                return current().value === value;
            }
        }
    }

    function delimited<T>(
        start: { value?: string; type?: string },
        stop: { value?: string; type?: string },
        separator: { value?: string; type?: string },
        parser: () => T
    ): T[] {
        const args = [];

        eat(start);

        while (!is(stop)) {
            args.push(parser());
            if (!is(stop)) {
                eat(separator);
            }
        }

        eat(stop);

        return args;
    }

    function parseVariableDeclaration(): VariableDeclaration {
        eat({ type: "keyword", value: "var" });

        const name = eat({ type: "identifier" }).value;

        eat({ type: "colon" });

        const type = eat({ type: "type" }).value;
        let value = undefined;

        // Need to implement parseExpression
        // if (is({ type: "operator", value: "=" })) {
        //     value = parseExpression();
        // }

        return new VariableDeclaration({
            name: name,
            type: type,
            value: value
        });
    }

    function parseParameterDeclaration(): ParameterDeclaration {
        const name = eat({ type: "identifier" }).value;

        eat({ type: "colon" });

        const type = eat({ type: "type" }).value;

        return new ParameterDeclaration({
            name: name,
            type: type
        });
    }

    function parseImport(): ImportStatement {
        const names = delimited(
            { type: "keyword", value: "import" },
            { type: "keyword", value: "from" },
            { type: "comma", value: "," },
            (): string => eat({ type: "identifier" }).value
        );

        const module = eat({ type: "string" }).value;

        return new ImportStatement({
            module: module,
            names: names
        });
    }

    function parseFunction(): FunctionDeclarationStatement {
        const exported = is({ type: "keyword", value: "export" });

        if (exported) {
            eat({ type: "keyword", value: "export" });
        }

        eat({ type: "keyword", value: "function" });

        const name = eat({ type: "identifier" }).value;

        let args = [];

        if (is({ type: "leftParenthesis", value: "(" })) {
            args = delimited(
                { type: "leftParenthesis", value: "(" },
                { type: "rightParenthesis", value: ")" },
                { type: "comma", value: "," },
                () => parseParameterDeclaration()
            );
        }

        eat({ type: "colon" });

        let type = eat({ type: "type" }).value;

        const statements = [];

        while (!is({ type: "keyword", value: "end" })) {
            statements.push(parseStatement());
        }

        eat({ type: "keyword", value: "end" });

        return new FunctionDeclarationStatement({
            exported: exported,
            name: name,
            args: args,
            type: type,
            statements: statements
        });
    }

    function parseReturn(): ReturnStatement {
        const start = current().position.start;
        eat({ type: "keyword", value: "return" });

        // const value = parseExpression();

        eat({});

        const end = current().position.end;

        return new ReturnStatement(
            {
                value: new VariableAccess({ name: "" })
            });
    }

    function parseStatement(): Statement {
        switch (current().type) {
            case "keyword":
                switch (current().value) {
                    case "import":
                        return parseImport();
                    case "export":
                    case "function":
                        return parseFunction();
                    case "return":
                        return parseReturn();
                    case "var":
                        return parseVariableDeclaration();
                    // case "if":
                    //     return parseIfStatement();
                }
                break;
        }
    }

    const program = [];

    while (current()) {
        program.push(parseStatement());
    }

    return program;
}
