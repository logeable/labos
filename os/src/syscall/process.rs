use crate::batch::{self, run_next_app};

pub fn sys_exit(xstate: i32) -> ! {
    println!("[kernel] Application exited with code {}", xstate);
    run_next_app()
}

pub fn sys_taskinfo() -> isize {
    batch::APP_MANAGER.exclusive_access().print_app_info();
    0
}
