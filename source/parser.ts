import { Token, Tokenizer } from "https://deno.land/x/tokenizer/mod.ts";

interface ProgramNode {
    type: string;
}

type ExpressionNode =
    | LeftUnaryExpressionNode
    | BinaryExpressionNode
    | RightUnaryExpressionNode
    | IdentifierNode;

type StatementNode =
    | VariableDeclarationNode
    | VariableAssignmentNode
    | FunctionDeclarationStatementNode
    | FunctionCallStatementNode
    | IfStatementNode
    | ImportStatementNode;

type Program = StatementNode[];

interface VariableDeclarationNode extends ProgramNode {
    type: "variable declaration";
    name: string;
    value: ExpressionNode;
}

interface VariableAssignmentNode extends ProgramNode {
    type: "variable assignment";
    name: string;
    value: ExpressionNode;
}

interface IdentifierNode extends ProgramNode {
    type: "identifier";
    value: string;
}

interface BinaryExpressionNode extends ProgramNode {
    type: "binary expression";
    left: ExpressionNode;
    operator: string;
    right: ExpressionNode;
}

interface LeftUnaryExpressionNode extends ProgramNode {
    type: "left unary expression";
    left: ExpressionNode;
    operator: string;
}

interface RightUnaryExpressionNode extends ProgramNode {
    type: "right unary expression";
    operator: string;
    right: ExpressionNode;
}

interface FunctionDeclarationStatementNode extends ProgramNode {
    type: "function declaration statement";
    exported: boolean;
    name: string;
    args: string[];
    statements: StatementNode[];
}

interface FunctionCallStatementNode extends ProgramNode {
    type: "function call statement";
    name: string;
    args: ExpressionNode[];
}

interface IfStatementNode extends ProgramNode {
    type: "if statement";
    condition: ExpressionNode;
    then: StatementNode[];
    else: StatementNode[];
}

interface ImportStatementNode extends ProgramNode {
    type: "import statement";
    module: string;
    names: string[];
}

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
        return (
            (type && value && current().type === type && current().value === value) ||
            (type && current().type === type) ||
            (value && current().value === value)
        );
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

    function parseImport(): ImportStatementNode {
        const names = delimited(
            { type: "keyword", value: "import" },
            { type: "keyword", value: "from" },
            { type: "comma", value: "," },
            (): string => eat({ type: "identifier" }).value
        );

        return {
            type: "import statement",
            module: eat({ type: "string" }).value,
            names: names
        };
    }

    function parseFunction(): FunctionDeclarationStatementNode {
        const exported = is({ type: "keyword", value: "export" });

        eat({ type: "keyword", value: "function" });

        const name = eat({ type: "identifier" }).value;

        let args = [];

        if (is({ type: "left parenthesis", value: "(" })) {
            args = delimited(
                { type: "left parenthesis", value: "(" },
                { type: "right parenthesis", value: ")" },
                { type: "comma", value: "," },
                (): string => eat({ type: "identifier" }).value
            );
        }

        const statements = [];

        while (!is({ type: "keyword", value: "end" })) {
            statements.push(parseStatement());
        }

        eat({ type: "keyword", value: "end" });

        return {
            type: "function declaration statement",
            exported: exported,
            name: name,
            args: args,
            statements: statements
        };
    }

    function parseStatement(): StatementNode {
        switch (current().type) {
            case "keyword":
                switch (current().value) {
                    case "import":
                        return parseImport();
                    case "export":
                    case "function":
                        return parseFunction();
                }
                break;
            case "identifier":
                break;
        }
    }

    const program = [];

    while (current()) {
        program.push(parseStatement());
    }

    return program;
}
