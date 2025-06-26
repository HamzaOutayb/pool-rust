// pub mod messenger;
use std::{cell::RefCell, rc::Rc};

use crate::Worker;
pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

pub struct Tracker<'a> {
    pub logger: &'a Worker,
    pub value: i32,
    pub max: i32,
}

impl<'a> Tracker<'a> {
    pub fn new(logger: &'a Worker,max: i32)-> Self {
        Self {
            logger,
            value: 0,
            max,
        }
    }

    pub fn set_value<T>(&self, value: &Rc<T>) {
        let c = Rc::strong_count(value) as f64;
        let percentage = (c/self.max as f64)*100.0;
        if (percentage as u128) >= 100 {
            self.logger.error(&String::from("Error: you are over your quota!"));
        } else if (percentage as u128 )>= 70 &&( percentage as u128) < 100 {
            self.logger.warning(&String::from("Warning: you have used up over X% of your quota! Proceeds with precaution"));
        }
    }

    pub fn peek<T>(&self, peek: &Rc<T>) {
        let c = Rc::strong_count(peek) as f64;
        let mut percentage = c/(self.max.clone() as f64);
        percentage *= 100.0;        self.logger.info(&format!("Info: you are using up to {}% of your quota", percentage as u128));
    }
}