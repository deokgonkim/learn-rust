
fn print_tuple() -> bool {
    let tuple = (1, 'a', 3.14, "Word");
    // tuple can have multiple types of data
    // accessing tuple elements using . notation
    println!("{}", tuple.2);
    true
}

fn print_array() -> bool {
    // let array = [1, 'a', 3.14, "word"];
    // array can hold only single type of data
    // accessing array elements using [] notation
    let array = ['a', 'b', '😊', 'd'];
    println!("{}", array[2]);
    true
}

// fn print_all_tuple() -> bool {
//     let tuple = (1, 'a', 3.14, "Word");
//     // TODO tuple에는, for in 에서 사용할 수 있는 것이 없나?
//     for val in tuple {
//         println!("{}", val);
//     }
//     true
// }

fn print_all_array() -> bool {
    let array = ['a', 'b', '😊', 'd'];
    // array can be used with `for` `in` loop
    for val in array {
        println!("{}", val);
    }
    true
}

fn main() {
    // print_tuple();
    // print_array();
    print_all_tuple();
    // print_all_array();
}

