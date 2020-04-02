import { readFileStr } from "https://deno.land/std/fs/mod.ts";;
import { WhistleTokenizer } from "./parser/tokenizer.ts";
import { WhistleParser } from "./parser/parser.ts";
import { JsCompiler } from "./compiler/js/js_compiler.ts";

const source = await readFileStr(Deno.args[0]);

const program = new WhistleParser(new WhistleTokenizer().tokenize(source)).parse();

const js = new JsCompiler(program);

console.log(js.compile());
