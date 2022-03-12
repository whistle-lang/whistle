import { System } from "./system.js";
import { Crypto } from "./crypto.js";
export async function load(url: string) {
  const imports = {
    sys: {
      printInt: System.printInt, 
    },
    crypto: {
        randomUUID: Crypto.randomUUID,
    }
  };
  const module = await WebAssembly.compileStreaming(fetch(Deno === undefined? url : `file:///${Deno.cwd()}/${url}`));
  const instance = await WebAssembly.instantiate(module, imports);
  // deno-lint-ignore no-explicit-any
  return instance.exports as any;
}