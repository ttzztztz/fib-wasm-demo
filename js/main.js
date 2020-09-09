const wasm = require("../pkg/fib_wasm");

const answer = wasm.fib(12);
console.log(answer);
