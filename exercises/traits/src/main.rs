use traits::KoreanShortCut;
use traits::cat_petting;
use traits::Cat;

fn main() {
    let neo = KoreanShortCut {
        name: "Neo".to_string(),
        age: 1,
    };

    println!("Here is neo {:#?}", neo);
    // println!("Neo says {}", neo.say());
    cat_petting(&neo);
}
