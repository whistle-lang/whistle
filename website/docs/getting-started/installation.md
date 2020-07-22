---
id: installation
title: Installation
---

The first step to begin using *Whistle* is to install the *Whistle* cli which
works on windows, linux and mac. It is also possible to use *Whistle* from code
in any enviornment that can run modern javascript. For an example of usage from
code see [building a simple playground](guides/playground.md).


Installation of the *Whistle* cli currently only requires one prerequisite:
[Deno](https://deno.land/). Deno is a modern javascript runtime created by the
same guy that created Node.js that attempts to improve and develop it further
without having to work with the outdated codebase of Node.js.


Running the following command in the terminal will install the *Whistle* cli
which provides an easy to use interface to interact with the [tokenizer](internals/tokenizer.md),
[parser](internals/parser.md) and [compiler](internals/compiler.md):
```bash
$ deno install -Af https://raw.github.com/whistle-lang/whistle/master/cli/whistle.ts
```
