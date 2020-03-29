import { readFileStr } from "https://deno.land/std/fs/mod.ts";;
import { WhistleTokenizer } from "./parser/tokenizer.ts";
import { WhistleParser } from "./parser/parser.ts";

const source = await readFileStr(Deno.args[0]);

const program = new WhistleParser(new WhistleTokenizer().tokenize(source)).parse();

for (const node of program) {
  console.log(JSON.stringify(node, null, 2));
}
