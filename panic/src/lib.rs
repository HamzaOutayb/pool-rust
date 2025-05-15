use std::fs::{File};

pub fn open_file(s: &str) -> File {
    let file = File::open("foo.txt").unwrap();
    file
}