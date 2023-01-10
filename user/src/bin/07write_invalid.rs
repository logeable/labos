#![no_main]
#![no_std]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let addr = 0x8020c000usize;

    let buf = unsafe { core::slice::from_raw_parts(addr as *const u8, 9) };

    user_lib::write(user_lib::console::STDOUT, buf);
    0
}
