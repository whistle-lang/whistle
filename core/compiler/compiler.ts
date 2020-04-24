import { Program } from "../parser/program.ts";

export type WhistleCompiler = (program: Program) => string;
