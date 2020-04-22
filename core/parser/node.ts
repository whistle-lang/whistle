import { WhistleParser } from "./parser.ts";

export interface SerializedNode<T> {
  type: string;
  value: T | SerializedNode<T>;
}

export abstract class Node<T> {
  public readonly value: T;

  constructor(value: T) {
    this.value = value;
  }

  public static parse(parser: WhistleParser): Node<any> {
    throw "Cannot parse bare node";
  }

  public serialize(): SerializedNode<T> {
    return {
      type: this.constructor.name,
      value: this.value,
    };
  }
}
