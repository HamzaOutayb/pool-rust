use std::collections::HashMap;
use std::num::ParseFloatError;


pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
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
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        if argv.len() >= 2 {

        }
        let callback = self.flags.get(input).ok_or_else(|| "Unknown flag".to_string())?;

        callback(argv[0], argv[1]).map_err(|e| format!("Error: {}", e))
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let c = a.parse::<i32>().unwrap();
    let d = b.parse::<i32>().unwrap();
    let div: f64 = c as f64 /d as f64;
    Ok((div).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let c = a.parse::<i32>().unwrap();
    let d = b.parse::<i32>().unwrap();
    let rim: f64 = c as f64 %d as f64;
    Ok((rim).to_string())
}