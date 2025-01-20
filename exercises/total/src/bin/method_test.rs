/**
 * Borrowing 쉽지 않네...
 */

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn generate(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age: age,
        }
    }
    fn grow(&mut self) -> &mut Self {
        self.age += 1;
        self
    }
}

fn main() {
    let mut person = Person::generate("Deokgon", 30);
    println!("The person : {:?}", person);
    let grown_person = person.grow();
    println!("The grown person : {:?}", grown_person);
    grown_person.grow();
    println!("The person : {:#?}", person);
    // println!("The grown person : {:?}", grown_person);
}
