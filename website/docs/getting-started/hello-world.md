---
id: hello-world
title: Hello, World!
---

Now that you have installe *Whistle* on to writing your first program! The program
will print `Hello, World!` to the console as per tradition when writing your first
program in a new programming language.

## Writing and Running your first Whistle Program

The first step in creating a *Whistle* program is creating a new file with the
extension `.whi` to signify that it is a *Whistle* source file. This file will
be tokenized, parsed and finally compiled by the *Whistle*
[compiler](internals/compiler.md).


In this case a new file called `HelloWorld.whi` will be created with the following
content:

```
function Log(text: string): none {
  #(js) console.log(text);
}

{
  Log("Hello, World!")
}
```

All we need to do now to run the program is to run the following in your terminal:

```bash
$ whistle run HelloWorld.whi
```

And the following output should appear in your terminal:

```
Hello, World!
```

If `Hello, World!` did print, congratulations! You’ve officially written a
*Whistle* program. If it however did not print your installation most likely
did not succeed or your `PATH` does not include the `~\.deno\bin` directory.

## Explanation

The `Hello, World!` program shows quite a few of *Whistle*s design choices and
how to use some of the basic features of *Whistle*.


The first important piece of the program is the function declaration declared
using the `function` keyword. This tells the [parser](internals/parser.md) to
expect the name or so called `identifier` of the function. After this an optional
parenthesis enclosed part comes which specifies all of the parameter names and
their respective types. Before the last part which is the actual function body
statement the return type of the function is specified.

```
function Log(text: string): none {

}
```

When the function declared is supposed to be accessable from other files than
the one it is specified in the `export` keyword can be used to prefix the function
declaration like this: `export function example: none { ... }`.


"Tips" in *Whistle* are specified by using the `#( ) ...` syntax (or `#( ) { ... }#`
for inline or multiline "tips"). They are a way of telling the compiler things
about your code such as telling it to insert the raw javascript code directly into
a program compiled to javascript.

```
#(js) console.log(text);
```

In our case this tells the compiler to call the javascript method `console.log`
directly to print our text parameter to console.


Finally we come to the last part of the `Hello, World!` example, the anonomyous
code block containing code that we want to run as soon as the program loads.
This is specified by enclosing multiple statements in braces (`{` and `}`).
Only anonomyous code blocks are automatically executed at run time while codeblocks
following for example function declarations are not executed.

```
{
  Log("Hello, World!")
}
```

In our anonomyous code block we have a function call statement which calls the
function previously declared. It does this by first specifying the identifier of
the function to call (in this case `Log`) and then enclosing all of the parameters
to pass to the function in parethesis separated by commas. 
