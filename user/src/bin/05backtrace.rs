#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    call_a();
    0
}

fn call_a() {
    println!("this is a");
    call_b();
}

fn call_b() {
    println!("this is b");
    call_c();
}

fn call_c() {
    panic!("this is c");
}
