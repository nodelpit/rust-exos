async fn treatment(param: &str) -> Result<String, String> {
    if param.len() >= 3 {
        Ok("Ok".to_string())
    } else {
        Err("Too short".to_string())
    }
}

async fn second_treatment(param: &str) -> Result<String, String> {
    treatment(param).await?;
    Ok("Second treatment Ok".to_string())
}

#[tokio::main]
async fn main() {
    println!("{:?}", treatment("rs").await);
    println!("{:?}", treatment("hello").await);
    println!("\n");
    println!("{:?}", second_treatment("rs").await);
    println!("{:?}", second_treatment("rust").await);
}
