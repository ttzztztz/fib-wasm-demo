const fs = require('fs')
const wasmFile = fs.readFileSync('./fib.wasm')

const wasm = new WebAssembly.Module(wasmFile, {});
const inst =  new WebAssembly.Instance(wasm, {
  env: {
    memoryBase: 512,
    tableBase: 256,
    memory: new WebAssembly.Memory({
      initial: 512,
      maximum: 2048,
    }),
    table: new WebAssembly.Table({
      initial: 256,
      maximum: 1024,
      element: 'anyfunc',
    }),
    abort: console.log,
  },
}).exports;

console.log(inst)