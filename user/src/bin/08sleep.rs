#![no_main]
#![no_std]

use user_lib::{get_time, yield_};

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("before sleep");
    let current_timer = get_time();
    let wait_for = current_timer + 3000000;
    while get_time() < wait_for {
        yield_();
    }
    println!("Test sleep OK!");
    0
}
