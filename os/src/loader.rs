use crate::{
    config::{APP_BASE_ADDRESS, APP_SIZE_LIMIT, KERNEL_STACK_SIZE, MAX_APP_NUM, USER_STACK_SIZE},
    trap::TrapContext,
};
use core::{arch::asm, slice};

#[repr(align(4096))]
#[derive(Clone, Copy)]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    fn push_context(&self, cx: TrapContext) -> usize {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        cx_ptr as usize
    }
}

#[repr(align(4096))]
#[derive(Clone, Copy)]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_NUM];

static USER_STACK: [UserStack; MAX_APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

pub fn load_apps() {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    unsafe {
        asm!("fence.i");
    }
    for i in 0..num_app {
        let base_i = get_base_i(i);
        (base_i..base_i + APP_SIZE_LIMIT)
            .for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });
        let src = unsafe {
            slice::from_raw_parts(app_start[i] as *const u8, app_start[i + 1] - app_start[i])
        };
        let dst = unsafe { slice::from_raw_parts_mut(base_i as *mut u8, src.len()) };
        dst.copy_from_slice(src);
    }
}

fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}

pub fn get_num_app() -> usize {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    unsafe { num_app_ptr.read_volatile() }
}

pub fn init_app_cx(app_id: usize) -> usize {
    KERNEL_STACK[app_id].push_context(TrapContext::app_init_context(
        get_base_i(app_id),
        USER_STACK[app_id].get_sp(),
    ))
}

pub fn get_app_data(app_id: usize) -> &'static [u8] {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    assert!(app_id < num_app);
    let app_start = unsafe { slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    unsafe {
        slice::from_raw_parts(
            app_start[app_id] as *const u8,
            app_start[app_id + 1] - app_start[app_id],
        )
    }
}
