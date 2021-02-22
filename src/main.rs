/*
global system allocator: 2017/04/17
https://doc.rust-lang.org/beta/unstable-book/language-features/global-allocator.html

the standard library has one “global” memory allocator: 2018/06/15 ver. 1.28.0-nightly
https://doc.rust-lang.org/nightly/std/alloc/index.html

jemalloc is removed by default
https://blog.rust-lang.org/2019/01/17/Rust-1.32.0.html#jemalloc-is-removed-by-default
*/
#![cfg_attr(
    has_global_allocator,
    feature(global_allocator, allocator_api, heap_api)
)]
#[cfg(has_global_allocator)]
#[global_allocator]
static GLOBAL: std::heap::System = std::heap::System;

#[cfg(has_std_alloc)]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;

use libaki_gsub::execute;

use runnel::RunnelIoeBuilder;

use std::io::Write;

fn main() {
    let mut env_args: Vec<String> = std::env::args().collect();
    let _program = env_args.remove(0);
    let program = env!("CARGO_PKG_NAME");
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    //
    let sioe = RunnelIoeBuilder::new().build();
    //
    match execute(&sioe, &program, &env_args) {
        Ok(_) => {}
        Err(err) => {
            #[rustfmt::skip]
            let _ = sioe.perr().lock()
                .write_fmt(format_args!("{}: {}\n", program, err));
            std::process::exit(1);
        }
    };
    //
    std::process::exit(0);
}
