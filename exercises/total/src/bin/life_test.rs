/*
 * When the function returns reference type.
 * the lifetime specifier may required.
 * 'a : lifetime specifier
 */


fn longer_str<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let str1 = "Short";
    let str2 = "Long";
    println!("{} is longer", longer_str(str1, str2));
}

