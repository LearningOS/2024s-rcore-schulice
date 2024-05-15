//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::sync::Arc;
use alloc::vec::Vec;
use lazy_static::*;
use core::cmp::Ordering;
use core::ops::{Add, AddAssign};
///A array of `TaskControlBlock` that is thread-safe
#[derive(Default)]
pub struct TaskManager {
    ready_vec: Vec<Arc<TaskControlBlock>>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self::default()
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_vec.push(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        if self.ready_vec.is_empty() {
            return None;
        }
        let mut idx = 0;
        for i in 1..self.ready_vec.len() {
            if self.ready_vec[i].inner_exclusive_access().stride < self.ready_vec[idx].inner_exclusive_access().stride {
                idx = i;
            }
        }
        let res = self.ready_vec.remove(idx);
        let mut inner = res.inner_exclusive_access();
        let prio = inner.priority;
        inner.stride += Stride(u64::MAX / prio as u64);
        drop(inner);
        Some(res)
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}

#[derive(Clone, Debug, Default)]
pub struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let mut res = self.0 < other.0;
        if u64::abs_diff(self.0, other.0) > u64::MAX / 2 {
            res = !res;
        }
        if res {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PartialEq for Stride {
    #[allow(unused_variables)]
    fn eq(&self, other: &Self) -> bool {
        false
    }
}

impl Add for Stride {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl AddAssign for Stride {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0);
    }    
}

impl From<usize> for Stride {
    fn from(value: usize) -> Self {
        Self(value as u64)
    }
}

impl Stride {
    pub fn new() -> Self {
        Self::default()
    }
}