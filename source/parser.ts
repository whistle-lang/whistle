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
    | FunctionCallStatementNode;

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
    name: string;
    args: IdentifierNode[];
    statements: StatementNode[];
}

interface FunctionCallStatementNode extends ProgramNode {
    type: "function call statement";
    name: string;
    args: ExpressionNode[];
}

interface ifStatementNode extends ProgramNode {
    type: "if statement";
    condition: ExpressionNode;
    then: StatementNode[];
    else: StatementNode[];
}
