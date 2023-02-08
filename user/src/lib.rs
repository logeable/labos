#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

use heap_allocator::init_heap;
use syscall::{
    sys_exec, sys_exit, sys_fork, sys_get_time, sys_read, sys_waitpid, sys_write, sys_yield,
};

#[macro_use]
pub mod console;
mod heap_allocator;
mod lang_items;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    init_heap();
    exit(main());
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}

pub fn exit(exit_code: i32) -> ! {
    sys_exit(exit_code);
    unreachable!();
}

pub fn yield_() -> isize {
    sys_yield()
}

pub fn get_time() -> isize {
    sys_get_time()
}

pub fn fork() -> isize {
    sys_fork()
}

pub fn exec(path: &str) -> isize {
    sys_exec(path)
}

pub fn wait(exit_code: &mut i32) -> isize {
    waitpid(-1, exit_code)
}

pub fn waitpid(pid: isize, exit_code: &mut i32) -> isize {
    loop {
        match sys_waitpid(pid as isize, exit_code) {
            -2 => {
                yield_();
            }
            exit_pid => return exit_pid,
        }
    }
}

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}
