use core::{arch::asm, panic::PanicInfo, ptr};

use crate::exit;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "\x1b[31mPanicked at {}:{} {}\x1b[0m",
            location.file(),
            location.line(),
            info.message().unwrap(),
        );
    } else {
        println!("\x1b[31mPanicked: {}\x1b[0m", info.message().unwrap());
    }
    unsafe {
        print_stack_trace();
    }
    exit(1);
}

unsafe fn print_stack_trace() -> () {
    let mut fp: *const usize;
    asm!("mv {}, fp", out(reg) fp);
    println!("== Begin stack trace [User mode] ==");
    while fp != ptr::null() {
        let saved_ra = *fp.sub(1);
        let saved_fp = *fp.sub(2);
        println!("ra = {:#016x}, fp = {:#016x}", saved_ra, saved_fp);
        fp = saved_fp as *const usize;
    }
    println!("== End stack tarce [User mode] ==");
}
