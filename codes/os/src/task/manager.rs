use super::TaskControlBlock;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use spin::Mutex;
use lazy_static::*;
use super::processor::*;

pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    pub fn new() -> Self {
        Self { ready_queue: VecDeque::new(), }
    }
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_queue.pop_front()
    }
}

lazy_static! {
    pub static ref TASK_MANAGER: Mutex<TaskManager> = Mutex::new(TaskManager::new());
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.lock().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    // println!("core{}:fetch task",get_core_id());
    TASK_MANAGER.lock().fetch()
}

pub fn find_task(pid:usize)->Option<Arc<TaskControlBlock>>{
    let inner = TASK_MANAGER.lock();
    for task in &inner.ready_queue {
        if task.pid.0 == pid {
            return Some(task.clone())
        }
    }
    return None
}