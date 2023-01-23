#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
mod console;
mod config;
mod lang_items;
mod loader;
mod logger;
mod mm;
mod sbi;
mod stack_trace;
mod sync;
mod syscall;
mod task;
mod timer;
mod trap;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    print_layout();

    mm::init();
    logger::init().unwrap();
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();

    task::run_first_task();

    unreachable!()
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
