extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
    let _mod: i64 = 1_000_000_007;

    let mut num1: i64 = 1;
    let mut num2: i64 = 1;

    for _ in 0..n {
        let cur: i64 = (num1 + num2) % _mod;
        num1 = num2;
        num2 = cur;
    }

    return ((num1 + num2) % _mod) as i32;
}