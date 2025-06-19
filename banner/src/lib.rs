use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag {
    pub short_hand: String,

    pub long_hand: String,

    pub desc: String,
}

impl<'a> Flag {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        Self {
            short_hand: format!("-{}", &name[0..1]),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
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
        self.flags.insert(flag.long_hand.clone(), func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
         let func = self.flags.get(input);
        match func {
            Some(f) => {
                let res = f(argv[0], argv[1]);
                match res {
                    Ok(res) => Ok(res),
                    Err(_) => Err("invalid float literal".to_string())
                }
            }
            None => Err("invalid float literal".to_string())
    }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let c = a.parse::<f64>()?;
    let d = b.parse::<f64>()?;
    let div: f64 = c as f64 /d as f64;
    Ok((div).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let c = a.parse::<f64>()?;
    let d = b.parse::<f64>()?;
    let rim: f64 = c as f64 %d as f64;
    Ok((rim).to_string())
}