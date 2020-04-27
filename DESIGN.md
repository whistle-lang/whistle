# Whistle Language Design

## Introduction

## Design

### Comments

Whistle is in no way unique in the way it defines it's comments. There is
single and multi line comments that use slashes and slashed combined with stars
as following:

```
// Single line comment

/*
  Multi line comment
*/
```

### Literals

In Whistle there are six base literals:

* Integer literal - `1234567890`
* Float literal - `1234567890.1234567890`
* Character literal - `'a'`
* String literal - `"string"`
* Boolean literal - `true` or `false`
* None literal - `none`

### Types

#### Base Types

There is currently eight base types in Whistle:

* `i32` - A 32 bit signed integer. If not specified the integer literal will default to `i32`.
* `i64` - A 64 bit signed integer.
* `f32` - A 32 bit floating point number. If not specified the float literal will default to `f32`.
* `f64` - A 64 bit floating point number.
* `string` - A string of chars
* `char` - A single char
* `bool` - A simple boolean type. Either `true` or `false`. Compiled to `0` or `1`.
* `none` - An universal type for stuff like `null`, `void`, `undefined` and `nan`

#### The `type` keyword

Todo

### Values & Variables

In Whistle there are two types of variables:

* `val` - Values use the `val` keyword and are immutable
* `var` - Varaibles use the `var` keyword and are mutable

#### Declaration

The declaration of values and variables is the same and is done as following:

```
val immutable_value: i32 = 123456789
var mutable_variable: i32 = 1234567890
```

#### Variable Assignment

Only variables can be reassigned to a different value. This new value must be
of the same type as the previous and is done as following:

```
var mutable_variable: i32 = 0

mutable_variable = 1
```

### Imports

In Whistle imports are done with the following syntax:

```
import a from "b.whi"
import c, d from "e.whi"
```

### Functions

Functions in Whistle are defined using the `function` keyword. They require a
list of parameters if there are any and it's return type. A simple function
could be declared as following:

```
// A function with no parameters returning none
function A: none { }

// A function with two parameters of type i32 returning an i32 of the two input
// parameters added together
function Add(a: i32, b: i32): i32 {
  return a + b
}
```

#### Exports

Exporting functions in Whistle is done with the `export` keyword as following:

```
export function ExportedFunction: none { }
```

### Control Flow

#### If Expression

The `if` expression in Whistle allows you to branch your code depending on the
condition. If the condition is met certain code will run otherwise if an `else`
keyword is supplied that code will run. It will then continue with normal execution.
Some valid `if` statements:

```
// A super simple if statement
if true
  DoStuff()

// A bit more advanced if statement
if a == b && b == c {
  DoSomething()
} else {
  DoSomethingElse()
}

// A chained if statement
if a == b {
  AIsB()
} else if b == c {
  BIsC()
}
```

#### Loops

There are currently two types of loops in Whistle defined by the following
keywords: `loop` and `while`.

##### `loop`

The `loop` keyword defines an infinte loop that will execute code until you
tell it to stop, usually with the `break` or `return` keyword. A `loop` loop
will probably look something like this:

```
loop {
  Again()
}
```

##### `while`

The `while` keyword defines an loop that runs while the condition is true or
is told to stop like the `loop` loop. An example `while` loop that runs while
the condition is true could look something like this:

```
var i = 0

while i < 3 {
  Print(i)
  i = i + 1
}
```
