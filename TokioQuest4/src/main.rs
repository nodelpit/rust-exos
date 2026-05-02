use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Start main");

    let h = tokio::spawn(async {
        sleep(Duration::from_secs(2)).await;
        println!("End of task");
    });

    println!("main continue while task running");
    let _ = h.await;
    println!("End main");
}