import { readFileStr } from "https://deno.land/std/fs/mod.ts";

import { WhistleLexer } from "./lexer.ts";

const source = await readFileStr(Deno.args[0]);

const lexer = new WhistleLexer(source);

for (const token of lexer) {
    console.log(token);
}
