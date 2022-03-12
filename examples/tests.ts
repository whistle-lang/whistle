import { assertEquals } from "https://deno.land/std/testing/asserts.ts";
import { load } from "../helper/mod.ts";


Deno.test("add", async () => {
    const { add } = await load("./add.wasm");
    assertEquals(add(1,3), 4);
});

Deno.test("assign", async () => {
    const { assign } = await load("./assign.wasm");
    assertEquals(assign(), 188);
});
Deno.test("fac", async () => {
    const { fac } = await load("./fac.wasm");
    assertEquals(fac(BigInt(33)), 1n);
});
Deno.test("funs", async () => {
    const { a, b, c, d } = await load("./funs.wasm");
    assertEquals(a(), 1);
    assertEquals(b(), 2);
    assertEquals(c(), 3);
    assertEquals(d(), 4);
});
Deno.test("array", async () => {
    const { testarray } = await load("./array.wasm");
    assertEquals(testarray(), [1,2,3]);
});