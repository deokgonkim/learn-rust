
pub trait Cat {
    fn say(&self) -> String;
}

#[derive(Debug)]
pub struct KoreanShortCut {
    pub name: String,
    pub age: u32,
}

impl Cat for KoreanShortCut {
    fn say(self: &Self) -> String {
        "묘~".to_string()
    }
}

pub fn cat_petting(some_cat: &impl Cat) {
    println!("Cat says {}", some_cat.say());
}
