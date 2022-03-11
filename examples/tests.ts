import { assertEquals } from "https://deno.land/std/testing/asserts.ts";
import { load } from "../helper/deno.ts";


Deno.test("add", async () => {
    const { add } = await load("./add.wasm");
    assertEquals(add(1,3), 3);
});
