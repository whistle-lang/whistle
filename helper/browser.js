import { System } from "./system.js";

export async function load(url) {
  const imports = {
    sys: {
      memory: System.memory,
      js: System.js,
    },
  };
  const module = await WebAssembly.compileStreaming(fetch(url));
  const instance = await WebAssembly.instantiate(module, imports);
  return instance.exports;
}