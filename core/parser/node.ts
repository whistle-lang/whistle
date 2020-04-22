import { WhistleParser } from "./parser.ts";

export interface Node<T> {
  type: string;
  value: T;
}

export type NodeParser<T extends Node<any>> = (
  parser: WhistleParser,
  ...params: any[]
) => T;
