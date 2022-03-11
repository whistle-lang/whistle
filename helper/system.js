export class System {
  static memory = new WebAssembly.Memory({ initial: 256, maximum: 256 });

  static js(offset, length) {
    Function(
      new TextDecoder("utf-8").decode(
        new Uint8Array(System.memory.buffer, offset, length),
      ),
    )();
  }
}
