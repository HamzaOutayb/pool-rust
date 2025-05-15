use std::path::Path;
use std::fs::{File};
use std::io::Write;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let file = OpenOptions::new().write(true).append(true).create(true).open(path);
    file.unwrap().Write(content.as_bytes()).expect("hh");
}