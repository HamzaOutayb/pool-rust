use std::{collections::HashMap, num::ParseFloatError};
pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl<'a> Flag <'a> {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
            Self {
                short_hand: format!("-{}", &name[0..1]),
                long_hand: format!("--{}", name),
                desc: desc.to_string(),
            }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand.clone(), func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let callback = match self.get(input) {
            Some(output),
            None,
        }
        Callback(argv[0], arg1[1])
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a = arg1.parse::<i32>().unwrap_err()
    let b = arg2.parse::<i32>().unwrap_err()
    (a/b).to_string()
}


pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a = arg1.parse::<i32>().unwrap_err()
    let b = arg2.parse::<i32>().unwrap_err()
    (a%b).to_string()
}