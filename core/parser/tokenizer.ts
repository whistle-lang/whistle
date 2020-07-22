import { Tokenizer } from "../tokenizer/tokenizer.ts";

export class WhistleTokenizer extends Tokenizer {
  constructor() {
    super([
      { type: "Whitespace", pattern: /\s+/, ignore: true },
      { type: "Comment", pattern: /\/\/.*/, ignore: true },
      { type: "Comment", pattern: /\/\*[.\n]*\*\//, ignore: true },

      {
        type: "Keyword",
        pattern: [
          "import",
          "from",
          "export",
          "function",
          "return",
          "if",
          "while",
          "loop",
          "break",
          "continue",
          "var",
          "val",
        ],
      },
      {
        type: "Type",
        pattern: ["i32", "i64", "f32", "f64", "string", "char", "bool", "none"],
      },

      {
        type: "Boolean",
        pattern: /true|false/,
        value: (m: { match: string }) => m.match === "true",
      },
      {
        type: "Float",
        pattern: /-?[0-9]+.[0-9]*/,
        value: (m: { match: string }) => parseFloat(m.match),
      },
      {
        type: "Integer",
        pattern: /-?[0-9]+/,
        value: (m: { match: string }) => parseInt(m.match),
      },
      {
        type: "Character",
        pattern: /'(\\?.)'/,
        value: (m: { groups: string[] }) => m.groups[0],
      },
      {
        type: "String",
        pattern: /"(.*?[^\\])"/,
        value: (m: { groups: string[] }) => m.groups[0],
      },

      { type: "Identifier", pattern: /[a-zA-Z_]+/ },

      {
        type: "Tip",
        pattern: /#\(([^)]*)\)\s*{((?:[^}]|})*)}#/
      },
      {
        type: "Tip",
        pattern: /#\(([^)]*)\)\s*(.*)/
      },

      { type: "LeftParenthesis", pattern: "(" },
      { type: "RightParenthesis", pattern: ")" },
      { type: "LeftBrace", pattern: "{" },
      { type: "RightBrace", pattern: "}" },
      { type: "Comma", pattern: "," },
      { type: "Colon", pattern: ":" },

      {
        type: "Operator",
        pattern: [
          "=",
          "!",
          "+",
          "-",
          "*",
          "/",
          "%",
          "==",
          "!=",
          "<",
          ">",
          "<=",
          ">=",
          "||",
          "&&",
        ],
      },
    ]);
  }
}
