use alloc::{collections::VecDeque, sync::Arc, vec::Vec};
use lazy_static::lazy_static;

use crate::{
    loader::{get_app_data, get_num_app},
    sync::UPSafeCell,
    trap::TrapContext,
};

use super::{
    context::TaskContext,
    switch::__switch,
    task::{TaskControlBlock, TaskStatus},
};

lazy_static! {
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }

    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().fetch()
}

// pub struct TaskManager {
//     num_app: usize,
//     inner: UPSafeCell<TaskManagerInner>,
// }

// impl TaskManager {
//     pub fn new() -> Self {
//         let num_app = get_num_app();

//         let mut tasks = Vec::new();
//         for i in 0..num_app {
//             tasks.push(TaskControlBlock::new(get_app_data(i), i))
//         }

//         Self {
//             num_app,
//             inner: unsafe { UPSafeCell::new(TaskManagerInner::new(tasks, 0)) },
//         }
//     }

//     pub fn mark_current_exited(&self) {
//         let mut inner = self.inner.exclusive_access();
//         let current = inner.current_task;
//         inner.tasks[current].task_status = TaskStatus::Exited;
//     }

//     pub fn mark_current_suspended(&self) {
//         let mut inner = self.inner.exclusive_access();
//         let current = inner.current_task;
//         inner.tasks[current].task_status = TaskStatus::Ready;
//     }

//     pub fn run_next_task(&self) {
//         if let Some(next) = self.find_next_task() {
//             let mut inner = self.inner.exclusive_access();
//             let current = inner.current_task;
//             inner.tasks[next].task_status = TaskStatus::Running;
//             inner.current_task = next;
//             let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
//             let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
//             drop(inner);

//             unsafe { __switch(current_task_cx_ptr, next_task_cx_ptr) };
//         } else {
//             panic!("All tasks completed!");
//         }
//     }

//     pub fn run_first_task(&self) {
//         let mut inner = self.inner.exclusive_access();
//         let task0 = &mut inner.tasks[0];
//         task0.task_status = TaskStatus::Running;
//         let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
//         drop(inner);

//         let mut _unused = TaskContext::zero_init();

//         unsafe {
//             __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
//         }
//     }

//     fn find_next_task(&self) -> Option<usize> {
//         let inner = self.inner.exclusive_access();
//         let current = inner.current_task;
//         (current + 1..current + 1 + self.num_app)
//             .map(|id| id % self.num_app)
//             .find(|&id| inner.tasks[id].task_status == TaskStatus::Ready)
//     }

//     pub fn get_current_token(&self) -> usize {
//         let inner = self.inner.exclusive_access();
//         inner.tasks[inner.current_task].get_user_token()
//     }

//     pub fn get_current_trap_cx(&self) -> &'static mut TrapContext {
//         let inner = self.inner.exclusive_access();
//         inner.tasks[inner.current_task].get_trap_cx()
//     }
// }

// struct TaskManagerInner {
//     tasks: Vec<TaskControlBlock>,
//     current_task: usize,
// }

// impl TaskManagerInner {
//     fn new(tasks: Vec<TaskControlBlock>, current_task: usize) -> Self {
//         Self {
//             tasks,
//             current_task,
//         }
//     }
// }
