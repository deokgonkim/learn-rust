use std::env;
use minigrep::run;
use minigrep::Config;

fn main() {
    let args = env::args();
    let mut params = Vec::new();
    for arg in args {
        params.push(arg);
    }
    let config = Config::build(&params).unwrap_or_else(|e| {
        panic!("Error: {e}")
    });
    dbg!(&config);
    let result = run(&config);
    if let Err(e) = result {
        panic!("Error: {e}")
    }
}

