import { readString } from "./utils.ts";

export class System {
  static memory?: WebAssembly.Memory;
  static printInt(arg: number) {
    console.log(arg);
  }
  static printString(arg: number) {
    console.log(readString(arg, System.memory!));
  }
}
