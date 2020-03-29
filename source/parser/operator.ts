import { Node } from "./node.ts";

export abstract class Operator<T extends string> extends Node<T> {}

export abstract class UnaryOperator<T extends string> extends Operator<T> {}

export abstract class BinaryOperator<T extends string> extends Operator<T> {
  abstract readonly precedence: number;
}
