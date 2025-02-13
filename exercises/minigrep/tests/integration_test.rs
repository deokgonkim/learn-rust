use minigrep;

mod common;

#[test]
fn test_run() {
    let config = common::setup();
    minigrep::run(&config);
}

