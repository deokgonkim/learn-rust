use minigrep;

pub fn setup() -> minigrep::Config {
    minigrep::Config::build(&vec!["this".to_string(), "test".to_string(), "./src/main.rs".to_string()]).expect("Failed to setup")
}

