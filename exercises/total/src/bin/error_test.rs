use std::io::{self, Read};
use std::fs::File;

fn read_string_from_file(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;
    Ok(string)
}

fn main() {
    let file_name = "./src/bin/error_test.rs";
    let result = read_string_from_file(&file_name);
    match result {
        Ok(string) => println!("Read from file \n{string}"),
        Err(e) => println!("Error {e}"),
    }
}

