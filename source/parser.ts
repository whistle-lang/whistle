import { Token } from "https://deno.land/x/tokenizer/mod.ts";

const precedence = {
    "=": 10,

    "==": 20,
    "!=": 20,
    "<": 20,
    ">": 20,
    "<=": 20,
    ">=": 20,

    "+": 30,
    "-": 30,
    "*": 30,
    "/": 30,
    "%": 30
};

abstract class Node<T> {
    public value: T;

    constructor(value: T) {
        this.value = value;
    }
}

type Expression =
    | BinaryExpression
    | VariableAccess
    | IntegerLiteral
    | FloatLiteral
    | StringLiteral
    | NoneLiteral;

type Statement =
    | VariableDeclaration
    | VariableAssignment
    | FunctionDeclarationStatement
    | FunctionCallStatement
    | IfStatement
    | ImportStatement
    | ReturnStatement
    | WhileStatement;

type Program = Statement[];

class IntegerLiteral extends Node<{
    value: number;
}> {}

class FloatLiteral extends Node<{
    value: number;
}> {}

class StringLiteral extends Node<{
    value: string;
}> {}

class NoneLiteral extends Node<{}> {}

class BinaryExpression extends Node<{
    left: Expression;
    operator: string;
    right: Expression;
}> {}

class VariableAccess extends Node<{
    name: string;
}> {}

class VariableDeclaration extends Node<{
    name: string;
    type: string;
    value: Expression;
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
    name: string;
    args: Expression[];
}> {}

class ReturnStatement extends Node<{
    value: Expression;
}> {}

class IfStatement extends Node<{
    condition: Expression;
    then: Statement[];
    else: Statement[];
}> {}

class WhileStatement extends Node<{
    condition: Expression;
    statements: Statement[];
}> {}

class ImportStatement extends Node<{
    module: string;
    names: string[];
}> {}

export function parse(tokens: Token[]): Program {
    let index = 0;

    const current = () => tokens[index];
    const next = () => tokens[index + 1];

    function eat({ value, type }: { value?: string; type?: string }, token: Token = current()) {
        if (!token) {
            throw `Unexpected empty token`;
        }

        if (type && value) {
            if (token.type !== type && token.value !== value) {
                throw `Expected type "${type}" with value "${value}" but recieved "${token.type}" with value "${token.value}" at index ${token.position.start}`;
            }

            if (token.type !== type) {
                throw `Expected type "${type}" but recieved "${token.type}" at index ${token.position.start}`;
            }

            if (token.value !== value) {
                throw `Expected value "${type}" but recieved "${token.type}" at index ${token.position.start}`;
            }
        } else {
            if (type) {
                if (token.type !== type) {
                    throw `Expected type "${type}" but recieved "${token.type}" at index ${token.position.start}`;
                }
            }
            if (value) {
                if (token.value !== value) {
                    throw `Expected value "${type}" but recieved "${token.type}" at index ${token.position.start}`;
                }
            }
        }

        return tokens[index++];
    }

    function is({ value, type }: { value?: string; type?: string }, token: Token = current()) {
        if (!token) {
            throw `Unexpected empty token`;
        }

        if (type && value) {
            return token.type === type && token.value === value;
        } else {
            if (type) {
                return token.type === type;
            }

            if (value) {
                return token.value === value;
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

    function parseExpression(): Expression {
        return parseBinaryOrExpression();
    }

    function parseBinaryOrExpression(left: Expression = parseAtom(), thisPrecedence: number = 0): Expression {
        if (is({ type: "operator" })) {
            const otherPrecedence = precedence[current().value];

            if (otherPrecedence > thisPrecedence) {
                const operator = eat({ type: "operator" }).value;

                const right = parseBinaryOrExpression(parseAtom(), otherPrecedence);

                const binary = new BinaryExpression({
                    left: left,
                    operator: operator,
                    right: right
                });

                return parseBinaryOrExpression(binary, thisPrecedence);
            }
        }
        
        return left;
    }

    function parseVariableAssignment(): VariableAssignment {
        const name = eat({ type: "identifier" }).value;

        eat({ type: "operator", value: "=" });

        const value = parseExpression();

        return new VariableAssignment({
            name: name,
            value: value
        });
    }

    function parseVariableDeclaration(): VariableDeclaration {
        eat({ type: "keyword", value: "var" });

        const name = eat({ type: "identifier" }).value;

        eat({ type: "colon" });

        const type = eat({ type: "type" }).value;
        let value = undefined;

        // Need to implement parseExpression
        if (is({ type: "operator", value: "=" })) {
            eat({ type: "operator", value: "=" });
            value = parseExpression();
        }

        return new VariableDeclaration({
            name: name,
            type: type,
            value: value
        });
    }

    function parseVariableAccess(): VariableAccess {
        return new VariableAccess({
            name: eat({ type: "identifer" }).value
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

    function parseFunctionDeclaration(): FunctionDeclarationStatement {
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

        while (!is({ type: "keyword", value: "***" })) {
            statements.push(parseExpression());
        }

        eat({ type: "keyword", value: "***" });

        return new FunctionDeclarationStatement({
            exported: exported,
            name: name,
            args: args,
            type: type,
            statements: statements
        });
    }

    function parseFunctionCall(): FunctionCallStatement {
        const name = eat({ type: "identifier" }).value;

        return new FunctionCallStatement({
            name: name,
            args: delimited(
                { type: "leftParenthesis", value: "(" },
                { type: "rightParenthesis", value: ")" },
                { type: "comma", value: "," },
                () => parseExpression()
            )
        });
    }

    function parseReturn(): ReturnStatement {
        eat({ type: "keyword", value: "return" });

        // const value = parseExpression();

        eat({});

        return new ReturnStatement({
            value: new VariableAccess({ name: "" })
        });
    }

    function parseIf(): IfStatement {
        eat({ type: "keyword", value: "if" });

        const condition = parseExpression();
        const thenStatements = [];

        while (!is({ type: "keyword", value: "***" })) {
            thenStatements.push(parseExpression());
        }

        const elseStatements = [];

        if (is({ type: "keyword", value: "else" })) {
            eat({ type: "keyword", value: "else" });

            while (!is({ type: "keyword", value: "***" })) {
                elseStatements.push(parseExpression());
            }
        }

        return new IfStatement({
            condition: condition,
            then: thenStatements,
            else: elseStatements
        });
    }

    function parseWhile(): WhileStatement {
        eat({ type: "keyword", value: "while" });

        const condition = parseExpression();
        const statements = [];

        while (!is({ type: "keyword", value: "*" })) {
            statements.push(parseExpression());
        }

        return new WhileStatement({
            condition: condition,
            statements: statements
        });
    }

    function parseAtom() {
        switch (current().type) {
            case "keyword":
                switch (current().value) {
                    case "import":
                        return parseImport();
                    case "export":
                    case "function":
                        return parseFunctionDeclaration();
                    case "return":
                        return parseReturn();
                    case "var":
                        return parseVariableDeclaration();
                    case "if":
                        return parseIf();
                    case "while":
                        return parseWhile();
                    case "none":
                        return new NoneLiteral({});
                }
            case "integer":
                return new IntegerLiteral({ value: eat({ type: "integer" }).value });
            case "float":
                return new FloatLiteral({ value: eat({ type: "float" }).value });
            case "string":
                return new StringLiteral({ value: eat({ type: "string" }).value });
            case "identifier":
                if (is({ type: "leftParenthesis" }, next())) {
                    return parseFunctionCall();
                } else if (is({ type: "operator", value: "=" }, next())) {
                    return parseVariableAssignment();
                } else {
                    return parseVariableAccess();
                }
            case "leftParenthesis":
                eat({ type: "leftParenthesis" });
                const expression = parseExpression();
                eat({ type: "rightParenthesis" });
                return expression;
        }
    }

    const program = [];

    while (current()) {
        program.push(parseAtom());
    }

    return program;
}
