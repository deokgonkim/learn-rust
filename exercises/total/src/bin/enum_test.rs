/**
 * 책에서는, string을 받고, enumerate 받을때 참조형으로 받는데,
 * 왜 그렇게 하는지 케이스를 찾지 못하겠네..
 */

fn with_char_type() {
    let array = ['a', 'b', 'c', 'd', 'e', '😊'];
    for (i, c) in array.iter().enumerate() {
        println!("Index: {}, Value: {}", i, c);
    }
}

fn with_str_type() {
    let array = ["a", "b", "c", "d", "e", "😊"];
    for (i, c) in array.iter().enumerate() {
        println!("Index: {}, Value: {}", i, c);
    }
}


fn with_string_type() {
    // define String array type
    type StringArray = [String; 6];
    let mut array: StringArray = Default::default();
    array[0] = String::from("a");
    array[1] = String::from("b");
    array[2] = String::from("c");
    array[3] = String::from("d");
    array[4] = String::from("e");
    array[5] = String::from("😊");
    for (i, c) in array.iter().enumerate() {
        println!("Index: {}, Value: {}", i, c);
    }
}

fn main() {
    with_string_type();
}
