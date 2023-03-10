use core::option_env;
use core::{arch::asm, ptr};

pub unsafe fn print_stack_trace() -> () {
    let trace = option_env!("TRACE").unwrap_or("false");
    if trace != "true" {
        return;
    }
    let mut fp: *const usize;
    asm!("mv {}, fp", out(reg) fp);
    println!("== Begin stack trace ==");
    while fp != ptr::null() {
        let saved_ra = *fp.sub(1);
        let saved_fp = *fp.sub(2);
        println!("ra = {:#016x}, fp = {:#016x}", saved_ra, saved_fp);
        fp = saved_fp as *const usize;
    }
    println!("== End stack tarce ==");
}
