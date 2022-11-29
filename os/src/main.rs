#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod lang_items;
mod logger;
mod sbi;

use core::arch::global_asm;
use log::{debug, error, info, trace, warn};
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logger::init().unwrap();

    print_layout();
    trace!("this is trace");
    debug!("this is debug");
    info!("this is info");
    warn!("this is warn");
    error!("this is error");

    warn!("shutdown now");
    sbi::shutdown();
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|x| unsafe { (x as *mut u8).write_volatile(0) })
}

fn print_layout() {
    extern "C" {
        fn skernel();
        fn stext();
        fn etext();

        fn sdata();
        fn edata();

        fn sstack();
        fn estack();
        fn sbss();
        fn ebss();
        fn ekernel();
        //fn rust_main();
    }
    println!(
        "kernel [{:#x}, {:#x}] rust_main = {:#x}",
        skernel as usize, ekernel as usize, rust_main as usize
    );
    println!(".text [{:#x}, {:#x}]", stext as usize, etext as usize);
    println!(".data [{:#x}, {:#x}]", sdata as usize, edata as usize);
    println!("stack [{:#x}, {:#x}]", sstack as usize, estack as usize);
    println!(".bss [{:#x}, {:#x}]", sbss as usize, ebss as usize);
}
