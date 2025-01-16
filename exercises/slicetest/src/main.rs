fn print_string_literal() {
    let str = "Hello, world!";
    println!("{}", str);
}

fn print_string() {
    let str = String::from("Hello, world!");
    println!("{}", str);
}

fn print_string_literal_slice() {
    let str = "Hello, world!";
    let slice = &str[0..5];
    println!("{}", slice);
}

fn print_string_slice() {
    let str = String::from("Hello, world!");
    let slice = &str[0..5];
    println!("{}", slice);
}

fn print_array_slice() {
    let a = ['a', 'b', 'c', 'd', 'e', '😊'];
    let sliced = &a[0..2];
    println!("Array: {}", a.iter().collect::<String>());
    println!("Sliced: {}", sliced.iter().collect::<String>());
}

fn print_array_slice2() {
    let a = ['a', 'b', 'c', 'd', 'e', '😊'];
    let sliced = &a[0..4];
    println!("First item in slice: {}", sliced[0]);
    println!("Sliced : {}", sliced.iter().collect::<String>());
    let mut merged_string = String::new();
    for c in sliced.iter() {
        merged_string.push_str(&c.to_string()[..]);
    }
    println!("Merged string: {}", merged_string);
}

fn main() {
    let mut command: String = String::from("");
    loop {
        println!("Enter a command:");
        std::io::stdin().read_line(&mut command).expect("Failed to read line");
        println!("You entered: {}", command.trim());
        match command.trim() {
            "1" => print_string_literal(),
            "2" => print_string(),
            "3" => print_string_literal_slice(),
            "4" => print_string_slice(),
            "5" => print_array_slice(),
            "6" => print_array_slice2(),
            "q" => break,
            _ => println!("Invalid command"),
        }
        command.clear();
    }
}
