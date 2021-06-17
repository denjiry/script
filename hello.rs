#!/usr/bin/env cargo-play
//# tokio = {version="1.7",features=["rt-multi-thread", "macros"]}
use std::future::Future;#[tokio::main]
async fn main() {
    dbg!(hage(hoge).await);
    dbg!(hage(hogeee).await);

}
async fn hoge() -> i32 {
    1

}
async fn hogeee() -> i32 {
    2

}
async fn hage<T, U>(func: impl Fn() -> T) -> U
where T: Future<Output=U>
{
    func().await

}
// rustc 1.55.0-nightly (a85f584ae 2021-06-16)
// [src/main.rs:7] hage(hoge).await = 1
// [src/main.rs:8] hage(hogeee).await = 2
