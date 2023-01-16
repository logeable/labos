use crate::{
    config::MAX_APP_NUM,
    loader::{get_num_app, init_app_cx},
    sync::UPSafeCell,
};

use super::{
    context::TaskContext,
    switch::__switch,
    task::{TaskControlBlock, TaskStatus},
};

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

impl TaskManager {
    pub fn new() -> Self {
        let num_app = get_num_app();

        let mut tasks = [TaskControlBlock {
            task_status: TaskStatus::UnInit,
            task_cx: TaskContext::zero_init(),
        }; MAX_APP_NUM];

        for i in 0..num_app {
            tasks[i].task_status = TaskStatus::Ready;
            tasks[i].task_cx = TaskContext::goto_restore(init_app_cx(i));
        }
        Self {
            num_app: MAX_APP_NUM,
            inner: unsafe { UPSafeCell::new(TaskManagerInner::new(tasks, 0)) },
        }
    }

    pub fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }

    pub fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }

    pub fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].task_status = TaskStatus::Running;
            inner.current_task = next;
            let current_task_cx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_cx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            drop(inner);

            println!("[kernel] run next app: {}", next);
            unsafe { __switch(current_task_cx_ptr, next_task_cx_ptr) };
        } else {
            panic!("All tasks completed!");
        }
    }

    pub fn run_first_task(&self) {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);

        let mut _unused = TaskContext::zero_init();

        println!("[kernel] run first app");
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + 1 + self.num_app)
            .map(|id| id % self.num_app)
            .find(|&id| inner.tasks[id].task_status == TaskStatus::Ready)
    }
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

impl TaskManagerInner {
    fn new(tasks: [TaskControlBlock; MAX_APP_NUM], current_task: usize) -> Self {
        Self {
            tasks,
            current_task,
        }
    }
}
