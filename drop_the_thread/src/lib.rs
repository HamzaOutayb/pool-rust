use core::panic;
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
        self.states.borrow_mut().push(false);
        (
            self.states.borrow().len()-1,
            Thread::new_thread(self.states.borrow().len()-1, c, self),
        )
    }

    pub fn track_worker(&self) -> usize {
        self.states.borrow().len()
    }

    pub fn is_dropped(&self, id: usize) -> bool {
        self.states.borrow_mut()[id]
    }

    pub fn add_drop(&self, id: usize) {
        if self.is_dropped(id) {
            println!("{}", id);
            panic!("{} is already dropped", id);
        }
            self.states.borrow_mut()[id] = true;
            self.drops.set(self.drops.get()+1);
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Thread<'a> {
    pid: usize,
    cmd: String,
    parent: &'a Workers,
}

impl<'a> Thread<'a> {
    pub fn new_thread(p: usize, c: String, t: &'a Workers) -> Self {
        Thread {
            pid: p,
            cmd: c,
            parent: t,
        }
    }

    pub fn skill(self) {
        self.parent.add_drop(self.pid);
    }
}

impl<'a> std::ops::Drop for Thread<'a> {
    fn drop(&mut self) {
        if !self.parent.is_dropped(self.pid) {
            self.parent.add_drop(self.pid);
        }
    }
}

