#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod batch;
mod lang_items;
mod logger;
mod sbi;
mod sync;
mod syscall;
mod trap;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logger::init().unwrap();
    trap::init();

    print_layout();
    batch::run_next_app();
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

        fn srodata();
        fn erodata();

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
    println!(".rodata [{:#x}, {:#x}]", srodata as usize, erodata as usize);
    println!(".data [{:#x}, {:#x}]", sdata as usize, edata as usize);
    println!("stack [{:#x}, {:#x}]", sstack as usize, estack as usize);
    println!(".bss [{:#x}, {:#x}]", sbss as usize, ebss as usize);
}
