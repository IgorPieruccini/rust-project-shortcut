use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read<'a>(input: &'a mut String) -> Vec<&'a str> {
    let path = Path::new("./projects_shortcut.txt");
    let mut file = File::open(path).expect("File not found");
    file.read_to_string(input)
        .expect("Error while reading file");

    return input.split("\n").collect();
}
