pub mod messenger;
pub use messenger::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::{rc::Rc};

#[derive(Clone)]
pub struct Worker {
    pub track_value: Rc<i32>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(track_value: i32) -> Self {
        Self {
            track_value: Rc::new(track_value),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(Vec::new()),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert(String::from("Warning"), msg.to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
    fn info(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert(String::from("Info"), msg.to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
    fn error(&self, msg: &str) {
        self.mapped_messages.borrow_mut().insert(String::from("Error"), msg.to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
}

