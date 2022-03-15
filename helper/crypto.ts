export class Crypto {
  static randomUUID() {
    return crypto.randomUUID();
  }
  static generateKeyAES(len: number) {
    return crypto.subtle.generateKey(
      {
        name: "AES-GCM",
        length: len,
      },
      true,
      ["encrypt", "decrypt"],
    );
  }
}
