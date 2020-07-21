import { NodeParser, Node } from "./node.ts";
import { WhistleParser } from "./parser.ts";

export interface Tip extends
  Node<{
    type: string,
    value: string
  }> {
  type: "Tip";
}

export const ParseTip: NodeParser<Tip> = (
  parser: WhistleParser,
) => {
  const token = parser.eat({ type: "Tip" });

  return {
    type: "Tip",
    value: {
      type: token.groups[0],
      value: token.groups[1]
    }
  };
};
