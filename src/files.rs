use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/**
 * Reads a file
 */

pub fn read_file(file: String) -> String {
    let mut f = File::open(file)
        .expect("Unable to use file!");

    let mut conts = String::new();
    f.read_to_string(&mut conts)
        .expect("Unable to use file!");

    return conts;
}

/**
 * Adds text to a file
 */
pub fn add_to_file(text: String, file: &str) {
    if Path::new(&file).exists() == true {
        let mut file = File::options()
            .read(true)
            .write(true)
            .open(file)
            .expect("Cannot create file!");

        file.write_all(text.as_bytes())
            .expect("Unable to write to a file!");
    } else {
        let mut file = File::create(file).expect("Unable to open file!");

        file.write_all(text.as_bytes())
            .expect("Unable to write to a file!");
    }
}