import { Tokenizer } from "https://deno.land/x/tokenizer/mod.ts";

export class WhistleLexer extends Tokenizer {
    constructor(source: string = "") {
        super(source, [
            { type: "whitespace", pattern: /\s+/, ignore: true },

            { type: "identifier", pattern: /[A-Z][a-zA-Z_]*/ },
            {
                type: "keyword",
                pattern: /(?:import|from|export|function|operator|return|if|while|end|i32|i64|f32|f64)/
            },

            { type: "left parenthesis", pattern: "(" },
            { type: "right parenthesis", pattern: ")" },
            { type: "left bracket", pattern: "[" },
            { type: "right bracket", pattern: "]" },
            { type: "comma", pattern: "," },
            { type: "colon", pattern: ":" },

            { type: "operator", pattern: /(?:-|\+|\/|\*|%|<|>|=|\?|-|\||~|&|#|@|£|\$|€|'|!)+/ },

            { type: "float", pattern: /-?[0-9]+.[0-9]*/, value: m => parseFloat(m) },
            { type: "integer", pattern: /-?[0-9]+/, value: m => parseInt(m) },
            { type: "string", pattern: /"(.*?[^\\])"/, value: m => m.groups[0] }
        ]);
    }
}
