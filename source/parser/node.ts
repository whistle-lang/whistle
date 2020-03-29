import { WhistleParser } from "./parser.ts";

export abstract class Node<T> {
  public readonly value: T;

  constructor(value: T) {
    this.value = value;
  }

  public static parse(parser: WhistleParser): Node<any> {
    throw "Cannot parse bare node";
  }
}
