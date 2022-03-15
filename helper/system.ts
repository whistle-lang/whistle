import { readString } from "./utils.ts";

export class System {
  static memory?: WebAssembly.Memory;
  static printInt(arg: number) {
    console.log(arg);
  }
  static printString(arg: number) {
    console.log(readString(arg, System.memory!));
  }
  static getDate() {
    return Date.now();
  }
  static getLocalStorageItem(arg: number) {
    return localStorage.getItem(readString(arg, System.memory!)!)!;
  }
  static setLocalStorageItem(arg1: number, arg2: number) {
    localStorage.setItem(readString(arg1, System.memory!)!, readString(arg2, System.memory!)!);
  }
}
