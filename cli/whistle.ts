import { Denomander, Whistle } from "./deps.ts";

const encoder = new TextEncoder();
const decoder = new TextDecoder();

const program = new Denomander({
  app_name: "Whistle CLI",
  app_description: "A CLI for the Whistle Programming Language",
  app_version: "0.1.0",
});

program
  .option("-p --pretty", "Pretty print the tokens/program");

program
  .command("tokenize [file]")
  .action(async (file: string) => {
    await Deno.stdout.write(
      encoder.encode(
        JSON.stringify(new Whistle().tokenize(
          decoder.decode(await Deno.readFile(file)),
        ), undefined, program.pretty ? 2 : undefined),
      ),
    );
  });

program
  .command("parse [file]")
  .action(async (file: string) => {
    await Deno.stdout.write(
      encoder.encode(
        JSON.stringify(new Whistle().parse(
          decoder.decode(await Deno.readFile(file)),
        ), undefined, program.pretty ? 2 : undefined),
      ),
    );
  });

// program
//   .command("compile [file]")
//   .action(async (file: string) => {
//     await Deno.stdout.write(
//       encoder.encode(
//         new Whistle().compile(
//           WhistleCompilerJs,
//           decoder.decode(await Deno.readFile(file)),
//         ),
//       ),
//     );
//   });

program.parse(Deno.args);
