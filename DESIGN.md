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
