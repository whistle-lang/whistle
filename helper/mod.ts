import { System } from "./system.ts";
import { Crypto } from "./crypto.ts";

function dirname(url: string) {
  return url.substring(0, url.lastIndexOf("/"));
}

export class Whistle {
  // deno-lint-ignore no-explicit-any
  instance: any;
  async load(url: string) {
    const imports = {
      sys: {
        printInt: System.printInt,
        printString: System.printString,
      },
      crypto: {
        randomUUID: Crypto.randomUUID,
      },
    };
    const module = await WebAssembly.compileStreaming(
      fetch(Deno === undefined ? url : `${dirname(Deno.mainModule)}/${url}`),
    );
    const instance = await WebAssembly.instantiate(module, imports);
    System.memory = (instance.exports.memory as WebAssembly.Memory) ||
      undefined;
    // deno-lint-ignore no-explicit-any
    this.instance = instance as any;
    if (this.instance.exports.main) {
      this.instance.exports.main();
    }
    // deno-lint-ignore no-explicit-any
    return instance.exports as any;
  }
}

export async function load(url: string) {
  return await new Whistle().load(url);
}
