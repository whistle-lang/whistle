<img src="logo.svg" width="100px" align="left" style="padding: 10px;" />
 
# Whistle

A dope new programming language that still doesnt have a std library. :sunglasses:

## CLI

### Installing

`deno install whistle -A -f https://raw.github.com/Ophyon/whistle/master/cli/whistle.ts`

### Usage

`whistle -h`

### Example

`whistle compile examples/HelloWorld.whi > HelloWorld.js`

the file `HelloWorld.js` should now look like this:

```js
function Log(text){console.log(text);}(() => {Log("Hello World");})();
```

`whistle run examples/HelloWorld.whi`
```
Hello World
```
