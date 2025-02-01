
// export submodule for public access
pub mod submodule;

pub mod inline_module {
    pub fn inline_func() {
        // println!("This is inline module");
        _inline_func();
    }

    fn _inline_func() {
        println!("This is inline module");
    }
}

// export main_library for public access
pub fn main_library() {
    println!("This is moduletest module");
}
