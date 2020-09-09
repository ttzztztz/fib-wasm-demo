const wasm = require("../pkg/fib_wasm");

const answer = wasm.fib(6);
console.log(answer);
