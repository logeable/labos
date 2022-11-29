use core::panic::PanicInfo;
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

    crate::sbi::shutdown()
}
