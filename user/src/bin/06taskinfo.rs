#![no_main]
#![no_std]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("should print taskinfo");
    user_lib::taskinfo();
    0
}
