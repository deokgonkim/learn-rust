use std::io;
use std::io::Error;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Config {
    file_path: String,
    text_to_find: String
}

impl Config {
    pub fn build(args: &Vec<String>) -> Result<Self, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        }
        let config = Config {
            file_path: args[2].clone(),
            text_to_find: args[1].clone()
        };
        Ok(config)
    }
}

pub fn run(config: &Config) -> Result<(), &str> {
    dbg!("Start Run");
    let file = File::open(&config.file_path);
    if let Err(e) = file {
        return Err("Unable to open file")
    }
    let mut string_read = String::new();
    let result = file.unwrap().read_to_string(&mut string_read);
    if let Err(e) = result {
        return Err("Unable to read file")
    }
    println!("Read from file\n{}", string_read);
    // for line in string_read.trim(). {
    //     println!("Line: {}", line);
    // }
    Ok(())
}

