<img src="logo.svg" width="100px" align="left" style="padding: 10px;" />
 
# Whistle

## CLI

### Installing

`deno install whistle -A -f https://raw.github.com/Ophyon/whistle/master/cli/whistle.ts`

### Usage

`whistle -h`

### Example

`whistle compile examples/HelloWorld.whi > HelloWorld.js`

the file `HelloWorld.js` should now look like this:

```js
import{Print}from"Console";function Main(){(Print("Hello World"));}
```

this wont work by default due to the std libraries not being properly implemented yet.
