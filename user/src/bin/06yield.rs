#![no_main]
#![no_std]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("before yield");
    user_lib::yield_();
    println!("after yield");
    0
}
