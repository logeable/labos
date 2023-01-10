mod fs;
mod process;

pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_TASKINFO: usize = 1000;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => fs::sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => process::sys_exit(args[0] as i32),
        SYSCALL_TASKINFO => process::sys_taskinfo(),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
