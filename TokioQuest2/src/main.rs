use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Attente de 2 secondes...");
    sleep(Duration::from_secs(2)).await;
    println!("Reprise (2s) !");
    sleep(Duration::from_secs(2)).await;
    println!("Attente terminée !");
}