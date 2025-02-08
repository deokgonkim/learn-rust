
pub trait Cat {
    fn say(self) -> String;
}

#[derive(Debug)]
pub struct KoreanShortCut {
    pub name: String,
    pub age: u32,
}

impl Cat for KoreanShortCut {
    fn say(self: Self) -> String {
        "묘~".to_string()
    }
}
