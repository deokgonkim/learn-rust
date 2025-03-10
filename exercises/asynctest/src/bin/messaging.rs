use trpl;
use core::time::Duration;


fn main() {
    let (tx, mut rx) = trpl::channel();
    tx.send("Aloha");

    let future0 = async {
        let future = async move {
            println!("Begun Future1");
            trpl::sleep(Duration::from_secs(5)).await;
            let _ = tx.send("Hello");
        };

        let future2 = async move {
            println!("Begun Future2");
            while let Some(value) = rx.recv().await {
                println!("Received {}", value);
            }
        };
        trpl::join(future, future2).await;
        // future.await;
        // future2.await;
    };
    trpl::run(
        future0
    );
}

