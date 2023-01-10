use crate::batch;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let check_result = batch::validate_addr(buf as usize, buf as usize + len);
    if !check_result {
        println!("[kernel] addr invalid!");
        batch::run_next_app();
    }
    match fd {
        FD_STDOUT => {
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize
        }
        _ => panic!("Unsupported fd = {} in sys_write!", fd),
    }
}
