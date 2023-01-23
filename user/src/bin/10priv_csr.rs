#![no_std]
#![no_main]

use riscv::register::sstatus::{self, SPP};

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("try to access privileged CSR in U mode");
    println!("kernel should kill this application!");
    unsafe {
        sstatus::set_spp(SPP::User);
    }
    0
}
