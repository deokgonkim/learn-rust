
#[derive(Debug)]
struct Cons(i32, Option<Box<Cons>>);

fn main() {
    let c = Cons(32, None);
    let d = Cons(32, Some(Box::new(c)));

    println!("c is {:#?}", d);
}

