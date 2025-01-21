/**
 * optional에 뭘 해봐야할지 모르겠네..
 */

fn print_optional_number() {
    let optional_number = Some(5);
    println!("This is the optional number: {:#?}", optional_number);
}

fn optional() {
    let optional_number: Option<i32> = None;
    // optional_number += 1;
    println!("This is the optional number: {:#?}", optional_number);
}

fn optional2() {
    let mut optional_number: Option<i32> = Some(1);
    optional_number += 1;
    println!("This is the optional number: {:#?}", new_number);
}

fn main() {
    // optional();
    optional2();
}
