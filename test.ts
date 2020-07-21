import { WhistleCompiler } from "./core/compiler/compiler.ts";
import { WhistleParser } from "./core/parser/parser.ts";
import { WhistleTokenizer } from "./core/parser/tokenizer.ts";
import { ParseProgram } from "./core/parser/program.ts";
import { readFileStr } from "https://deno.land/std/fs/mod.ts";
import { CompilationFile } from "./core/compiler/types.ts";

const files: Partial<CompilationFile>[] = [
    {
        filename: "Tip.whi",
    }
];

const tokenizer = new WhistleTokenizer();

for (const file of files) {
    if (file.filename) {
        const tokens = tokenizer.tokenize(await readFileStr(file.filename) + "\n");
        const program = ParseProgram(new WhistleParser(tokens));
        
        file.program = program;
    }
}

const compiler = new WhistleCompiler(files[0] as CompilationFile, [files[1] as CompilationFile]);

console.log(compiler.compile());
