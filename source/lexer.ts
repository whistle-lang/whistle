import { Tokenizer } from "https://deno.land/x/tokenizer/mod.ts";

export class WhistleLexer extends Tokenizer {
    constructor(source: string = "") {
        super(source, [
            { type: "whitespace", pattern: /\s+/, ignore: true },
            { type: "comment", pattern: /\/\/.*/, ignore: true },
            { type: "comment", pattern: /\/\*[.\n]*\*\//, ignore: true },

            {
                type: "keyword",
                pattern: ["import", "from", "export", "function", "return", "if", "while", "end", "var"]
            },
            {
                type: "type",
                pattern: ["i32", "i64", "f32", "f64", "none"]
            },
            { type: "identifier", pattern: /[a-zA-Z_]+/ },

            { type: "leftParenthesis", pattern: "(" },
            { type: "rightParenthesis", pattern: ")" },
            { type: "comma", pattern: "," },
            { type: "colon", pattern: ":" },

            {
                type: "operator",
                pattern: ["+", "-", "*", "/", "%", "=", "==", "!=", "<", ">", "<=", ">="]
            },

            { type: "float", pattern: /-?[0-9]+.[0-9]*/, value: m => parseFloat(m.match) },
            { type: "integer", pattern: /-?[0-9]+/, value: m => parseInt(m.match) },
            { type: "string", pattern: /"(.*?[^\\])"/, value: m => m.groups[0] }
        ]);
    }
}
