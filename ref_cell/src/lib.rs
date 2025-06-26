pub mod messenger;
pub use messenger::*;
pub use std::cell::RefCell;
pub use std::collections::HashMap;
pub use std::{rc::Rc};

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
        let s = msg.split(": ").collect::<Vec<&str>>();
        self.mapped_messages.borrow_mut().insert(String::from(s[0]), s[1].to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
    fn info(&self, msg: &str) {
        let s = msg.split(": ").collect::<Vec<&str>>();
        self.mapped_messages.borrow_mut().insert(String::from(s[0]), s[1].to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
    fn error(&self, msg: &str) {
        let s = msg.split(": ").collect::<Vec<&str>>();
        self.mapped_messages.borrow_mut().insert(String::from(s[0]), s[1].to_string());
        self.all_messages.borrow_mut().push(msg.to_string());
    }
}

