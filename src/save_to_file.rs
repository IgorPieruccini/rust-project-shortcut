use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn write<'a>(input: &'a String, path: &'a String) {
    let mut data_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .expect("Could not open file");

    data_file
        .write(input.as_bytes())
        .expect("Could not write to project file");
}
