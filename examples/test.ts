import { load } from "../helper/mod.ts";

const { helloworld } = await load("./helloworld.whi")
helloworld();