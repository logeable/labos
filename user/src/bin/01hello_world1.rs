#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    for i in 0..1000 {
        println!("01 hello world {}", i);
    }

    0
}
