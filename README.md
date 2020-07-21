<img src="logo.svg" width="100px" align="left" style="padding: 10px;" />
 
# Whistle

A dope new programming language that still doesnt have a std library. :sunglasses:

## CLI

### Installing

`deno install whistle -A -f https://raw.github.com/Ophyon/whistle/master/cli/whistle.ts`

### Usage

`whistle -h`

### Example

`whistle compile examples/Tip.whi > Tip.js`

the file `Tip.js` should now look like this:

```js
(() => {console.log("Single line tip");
        console.log("Multi line tip");
    })();

```
