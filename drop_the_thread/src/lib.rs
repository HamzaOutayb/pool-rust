pub use std::{
    cell::{Cell, RefCell},
    os::unix::thread,
};

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Workers {
    pub drops: Cell<usize>,
    pub states: RefCell<Vec<bool>>,
}

impl Workers {
    pub fn new() -> Workers {
        Self {
            drops: Cell::new(0),
            states: RefCell::new(vec![]),
        }
    }
    pub fn new_worker(&self, c: String) -> (usize, Thread) {
        (
            self.states.borrow().len(),
            Thread::new_thread(self.drops.get(), c, self),
        )
    }
    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }
    pub fn is_dropped(&self, id: usize) -> bool {
        if self.states.borrow().len() > id {
            return self.states.borrow_mut()[id];
        };
        false
    }
    pub fn add_drop(&self, id: usize) {
        self.states.borrow_mut().push(true);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pid: usize,
    cmd: String,
    parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Thread {
        Thread {
            pid: p,
            cmd: c,
            parent: t,
        }
    }

    pub fn skill(self) {
        if self.parent.states.borrow().len() > self.pid {
            self.parent.states.borrow_mut()[self.pid] = false;
        }
    }
}

trait Drop {
    fn add_drop(self);
}

impl<'a> Drop for Thread<'a> {
    fn add_drop(self) {
        self.parent.states.borrow_mut()[self.pid] = !self.parent.states.borrow_mut()[self.pid];
    }
}
