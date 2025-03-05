use trpl::sleep;
use tokio::time::Duration;


async fn test() {
    sleep(Duration::from_millis(5000)).await;
    println!("test done");
}

fn main() {
    println!("Hello World");
    trpl::run(
        test()
    );
    println!("Done");
    /*
        async {
            println!("Hello, world!");
            test().await;
            println!("Done");
        }
    */
}

