async fn start() {
    println!("démarrage de l’application");
}
async fn execute() {
    println!("exécution de la tâche asynchrone");
}

async fn stop() {
    println!("fin de l’application");
}

#[tokio::main]
async fn main() {
    start().await;
    execute().await;
    stop().await;
}