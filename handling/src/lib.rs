use std::path::Path;
use std::io::Write;
use std::fs::OpenOptions;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path);
    file.unwrap().write(content.as_bytes()).unwrap();
}