//! deadlock detector
use core::cell::RefMut;

use super::UPSafeCell;
use alloc::vec::*;
use alloc::vec;

/// use to detect deadlock
pub struct DeadlockDetector {
    inner: UPSafeCell<DeadlockDetectorInner>,
}

pub struct DeadlockDetectorInner {
    pub thread_num: u64,
    pub resource_num:u64,
    /// resid to get
    pub aval: Vec<u64>,
    /// [resid][tid]
    pub alloc: Vec<Vec<u64>>,
    /// [resid][tid]
    pub need: Vec<Vec<u64>>,
}

#[allow(dead_code)]
impl DeadlockDetector {
    /// init with one thread and process
    pub fn new() -> Self {
        Self {
            inner: unsafe { UPSafeCell::new( 
                DeadlockDetectorInner {
                    thread_num: 1,
                    resource_num: 0,
                    aval: vec![],
                    alloc: vec![],
                    need: vec![],
            })},
        }
    }
    /// inner 
    pub fn inner_exclusive_access(&self) -> RefMut<'_, DeadlockDetectorInner> {
        self.inner.exclusive_access()
    }
    /// add new thread to tail
    pub fn append_thread(&self) {
        let mut inner = self.inner.exclusive_access();
        for i in inner.alloc.iter_mut() {
            i.push(0);
        }
        for i in inner.need.iter_mut() {
            i.push(0);
        }
        inner.thread_num += 1;
    }
    /// add new resource
    pub fn append_resource(&self) {
        let mut inner = self.inner.exclusive_access();
        let thread_num = inner.thread_num;
        inner.alloc.push(vec![0; thread_num as usize]);
        inner.need.push(vec![0; thread_num as usize]);
        inner.aval.push(0);
        inner.resource_num += 1;
    }
}

#[allow(dead_code)]
impl DeadlockDetectorInner {
    pub fn is_safe(&self) -> bool {
        let mut work = self.aval.clone();
        let mut finish: Vec<bool> = vec![false; self.thread_num as usize];
        loop {
            let worker_tid = finish
                .iter().enumerate()
                .find(|(tid, &v)| {
                    !v && {
                        for (resource_id, &limit) in work.iter().enumerate() {
                            if self.need[resource_id][*tid] > limit {
                                return false;
                            }
                        }
                        true
                    }})
                .map(|(i, _)| i);
            if worker_tid.is_none() {
                break;
            }
            let worker_tid = worker_tid.unwrap();
            for (resource_id, item) in work.iter_mut().enumerate() {
                *item += self.alloc[resource_id][worker_tid];
            }
            finish[worker_tid] = true;
        }
        for &i in finish.iter() {
            if !i {
                return false;
            }
        }
        true
    }
    /// clear reasource
    pub fn clear_reasource(&mut self, id: usize) {
        assert!(id < self.resource_num as usize);
        for i in self.alloc.iter_mut() {
            i[id] = 0;
        }
        for i in self.need.iter_mut() {
            i[id] = 0;
        }
        self.aval[id] = 0;
    }
    /// clear task
    pub fn clear_tid(&mut self, id: usize) {
        assert!(id < self.thread_num as usize);
        for (idx, item) in self.aval.iter_mut().enumerate() {
            *item += self.alloc[idx][id];
            self.alloc[idx][id] = 0;
            self.need[idx][id] = 0;
        }
    }
    /// print info
    pub fn print(&self) {
        println!("[deadlock_detector]: thread: {}, resource: {}", self.thread_num, self.resource_num);
    }
}