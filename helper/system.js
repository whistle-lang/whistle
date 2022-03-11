export class System {
  static memory = new WebAssembly.Memory({ initial: 256, maximum: 256 });

  static println(offset, length) {
    console.log(
      new TextDecoder("utf-8").decode(
        new Uint8Array(System.memory.buffer, offset, length),
      ),
    );
  }
}
