use std::io;

fn main() {
    let readline: usize;
    let mut name = String::new();

    println!("Provide name");

    readline = io::stdin().read_line(&mut name).expect("Should provide name");

    let trimmed_name = name.trim();

    println!("User provided {} character name", readline - 1);

    println!("Hello {trimmed_name}, this is Rust world");
}
