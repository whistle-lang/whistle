<h1 align="center">
  <img src=".assets/whistle_horizontal_dark.svg" width="400px" align="center" />
</h1>

<p align="center">
  A dope new programming language that still doesnt have a std library. :sunglasses:
</p>

## CLI

### Installing

```base
$ deno install whistle -A -f https://raw.github.com/whistle-lang/whistle/master/cli/whistle.ts
```

### Usage

```
$ whistle -h
```

### Example

```
$ whistle compile examples/HelloWorld.whi > HelloWorld.js
```

the file `HelloWorld.js` should now look like this:

```js
function Log(text){console.log(text);}(() => {Log("Hello World");})();
```

and running:

```
$ whistle run examples/HelloWorld.whi
```

will produce this output:

```
Hello World
```
