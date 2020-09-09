/**
 * @deprecated
 */
const fib = (n) => {
    const mod = (1e9+7) | 0;
    let [x, y] = [1, 1];

    for (let i = 0; i < n; i++) {
        const nxt = (x + y) % mod;
        x = y;
        y = nxt;
    }

    return (x + y) % mod;
}

const $1 = new Date().getTime()
const js_answer = fib(10000000)
const $2 = new Date().getTime()

console.log(`js_answer = ${js_answer}, takes ${$2 - $1} ms to calculate`)

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

const $3 = new Date().getTime()
const wasm_answer = inst.fib(10000000)
const $4 = new Date().getTime()

console.log(`wasm_answer = ${wasm_answer}, takes ${$4 - $3} ms to calculate`)