use reqwest;
use trpl;
use std::env;

fn main() {
    trpl::run(async {
        let url = match env::args().last() {
            Some(url) => url,
            None => panic!("Error")
        };
        let resp = reqwest::get(url).await;
        println!("{resp:#?}")
    });
}

