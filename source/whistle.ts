const { args } = Deno;
import { parse } from "https://deno.land/std/flags/mod.ts";

console.dir(parse(args));
