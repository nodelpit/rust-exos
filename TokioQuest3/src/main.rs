use tokio::time::{sleep, Duration};

async fn func1() {
    println!("Start of first function (wait 3s)");
    sleep(Duration::from_secs(3)).await;
    println!("End of first function");
}

async fn func2() {
    println!("Start of second function (wait 2s)");
    sleep(Duration::from_secs(2)).await;
    println!("End of second function");
}

#[tokio::main]
async fn main() {

    // func1().await;
    // func2().await;

    tokio::join!(
        func1(),
        func2()
    );
}
