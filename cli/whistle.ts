import { Denomander, dirname, join, resolve, extname } from "./deps.ts";
import { WhistleTokenizer } from "../core/parser/tokenizer.ts";
import { WhistleParser } from "../core/parser/parser.ts";
import { ParseProgram, Program } from "../core/parser/program.ts";
import { WhistleCompiler } from "../core/compiler/compiler.ts";
import { CompilationFile, WhistleCompilationFile, JsCompilationFile } from "../core/compiler/types.ts";

const encoder = new TextEncoder();
const decoder = new TextDecoder();

const whistle = new Denomander({
  app_name: "Whistle CLI",
  app_description: "A CLI for the Whistle Programming Language",
  app_version: "0.1.0",
});

whistle
  .option("-p --pretty", "Pretty print the tokenized/parsed output");

whistle
  .command("tokenize [file]")
  .action(async ({ file }: { file: string}) => {
    const tokenizer = new WhistleTokenizer();
    const tokens = tokenizer.tokenize(
      decoder.decode(await Deno.readFile(file)),
    );

    await Deno.stdout.write(
      encoder.encode(
        JSON.stringify(
          tokens,
          undefined,
          whistle.pretty ? 2 : undefined,
        ),
      ),
    );
  });

whistle
  .command("parse [file]")
  .option("-p --pretty", "Pretty print the tokens/program")
  .action(async ({ file }: { file: string}) => {
    const tokenizer = new WhistleTokenizer();
    const tokens = tokenizer.tokenize(
      decoder.decode(await Deno.readFile(resolve(file))),
    );
    const parser = new WhistleParser(tokens);
    const program = ParseProgram(parser);

    await Deno.stdout.write(
      encoder.encode(
        JSON.stringify(
          program,
          undefined,
          whistle.pretty ? 2 : undefined,
        ),
      ),
    );
  });

async function findFiles(directory: string, entry: WhistleCompilationFile): Promise<CompilationFile[]> {
  const files: Set<CompilationFile> = new Set();
  const tokenizer = new WhistleTokenizer();

  for (const filename in WhistleCompiler.findImports(entry.program)) {
    const path = resolve(join(directory, filename));
    const extension = extname(path);
    const data = decoder.decode(await Deno.readFile(path));

    if (extension === ".whi") {
      const tokens = tokenizer.tokenize(data);
      const parser = new WhistleParser(tokens);
      const program = ParseProgram(parser);
      const file: WhistleCompilationFile = {
        language: "whistle",
        filename, program
      };
  
      files.add(file);
  
      for (const importedFile of await findFiles(dirname(path), file)) {
        files.add(importedFile);
      }
    }

    if (extension === ".js") {
      const file: JsCompilationFile = {
        language: "javascript",
        filename,
        content: data
      };

      files.add(file);
    }
  }

  return [...files];
}

whistle
  .command("compile [file]")
  .action(async ({ file }: { file: string}) => {
    const tokenizer = new WhistleTokenizer();
    const tokens = tokenizer.tokenize(decoder.decode(await Deno.readFile(resolve(file))));
    const parser = new WhistleParser(tokens);
    const program = ParseProgram(parser);
    const directory = dirname(resolve(file));
    const filename = resolve(file).replace(directory, "");
    const entry: WhistleCompilationFile = { language: "whistle", filename, program };
    const compiler = new WhistleCompiler(entry);
    const files = await findFiles(directory, entry);

    compiler.files.push(...files);

    await Deno.stdout.write(encoder.encode(compiler.compile()));
  });

whistle
  .command("run [file]")
  .action(async ({ file }: { file: string}) => {
    const tokenizer = new WhistleTokenizer();
    const tokens = tokenizer.tokenize(decoder.decode(await Deno.readFile(resolve(file))));
    const parser = new WhistleParser(tokens);
    const program = ParseProgram(parser);
    const directory = dirname(resolve(file));
    const filename = resolve(file).replace(directory, "");
    const entry: WhistleCompilationFile = { language: "whistle", filename, program };
    const compiler = new WhistleCompiler(entry);
    const files = await findFiles(directory, entry);

    compiler.files.push(...files);

    const source = compiler.compile();

    await Deno.run({
      cmd: ["deno", "eval", source]
    }).status();
  });

whistle.parse(Deno.args);
