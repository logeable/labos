use crate::timer::get_time_us;

pub fn sys_get_time() -> isize {
    get_time_us() as isize
}
