import { Tokenizer } from "../deps.ts";

export class WhistleTokenizer extends Tokenizer {
  constructor() {
    super([
      { type: "whitespace", pattern: /\s+/, ignore: true },
      { type: "comment", pattern: /\/\/.*/, ignore: true },
      { type: "comment", pattern: /\/\*[.\n]*\*\//, ignore: true },

      {
        type: "keyword",
        pattern: [
          "import",
          "from",
          "export",
          "function",
          "return",
          "if",
          "while",
          "var",
        ],
      },
      {
        type: "type",
        pattern: ["i32", "i64", "f32", "f64", "string", "char", "none"],
      },

      {
        type: "boolean",
        pattern: /true|false/,
        value: (m: { match: string }) => m.match === "true",
      },
      {
        type: "float",
        pattern: /-?[0-9]+.[0-9]*/,
        value: (m: { match: string }) => parseFloat(m.match),
      },
      {
        type: "integer",
        pattern: /-?[0-9]+/,
        value: (m: { match: string }) => parseInt(m.match),
      },
      {
        type: "character",
        pattern: /'(\\?.)'/,
        value: (m: { groups: string[] }) => m.groups[0],
      },
      {
        type: "string",
        pattern: /"(.*?[^\\])"/,
        value: (m: { groups: string[] }) => m.groups[0],
      },

      { type: "identifier", pattern: /[a-zA-Z_]+/ },

      { type: "leftParenthesis", pattern: "(" },
      { type: "rightParenthesis", pattern: ")" },
      { type: "leftBrace", pattern: "{" },
      { type: "rightBrace", pattern: "}" },
      { type: "comma", pattern: "," },
      { type: "colon", pattern: ":" },

      {
        type: "operator",
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
