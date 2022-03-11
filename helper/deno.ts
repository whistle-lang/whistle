import { System } from "./system.js";

export async function load(url: string) {
  const imports = {
    sys: {
      memory: System.memory,
      js: System.js,
    },
  };
  const module = await WebAssembly.compileStreaming(fetch(`file:///${Deno.cwd()}/${url}`));
  const instance = await WebAssembly.instantiate(module, imports);
  // deno-lint-ignore no-explicit-any
  return instance.exports as any;
}