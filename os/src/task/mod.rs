use lazy_static::lazy_static;

use manager::TaskManager;

use crate::trap::TrapContext;

mod context;
mod manager;
mod switch;
mod task;

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = TaskManager::new();
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task()
}

pub fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

pub fn current_user_token() -> usize {
    TASK_MANAGER.get_current_token()
}

pub fn current_trap_cx() -> &'static mut TrapContext {
    TASK_MANAGER.get_current_trap_cx()
}
