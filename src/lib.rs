extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Deserialize, Serialize, Clone)]
struct GithubFollowerInfo {
    login: String,
    id: u64,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn get_github_info(username: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("https://api.github.com/users/{username}/following?per_page={per_page}&page={page}",
        username = username,
        per_page = 100,
        page = 1);

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/vnd.github.v3+json");
    request.headers().set("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/84.0.4147.125 Safari/537.36");

    let window = web_sys::window().unwrap();
    let resp_promise = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_promise.is_instance_of::<Response>());
    let resp: Response = resp_promise.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}

#[wasm_bindgen]
pub fn dom_operation() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let container_dom = document.get_element_by_id("container").unwrap();
    container_dom.set_inner_html("hello, world!");

    log("dom opeation ok!!!");
}

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

    let answer: i32 = ((num1 + num2) % _mod) as i32;
    log("wasm ok!!!");
    return answer;
}