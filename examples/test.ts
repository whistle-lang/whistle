import { load } from "../../helper/mod.ts";

const { helloworld } = await load("examples/js/helloworld.wasm");
helloworld();