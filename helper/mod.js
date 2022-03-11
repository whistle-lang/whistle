import { System } from "./system.js";

export class Instance {
  constructor(instance, bridge = null) {
    const imports = bridge
      ? {
        sys: {
          memory: System.memory,
          println: System.println,
        },
        ...bridge,
      }
      : {
        sys: {
          memory: System.memory,
          println: System.println,
        },
      };
    WebAssembly.compileStreaming(fetch(instance)).then((module) => {
      WebAssembly.instantiate(module, imports).then((instance) => {
        this.#instance = instance;
      });
    });
  }
  get exports() {
    return this.#instance.exports;
  }
  get instance() {
    return this.#instance;
  }
}
