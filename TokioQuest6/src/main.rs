async fn receiver(param: &str) -> Result<String, String> {
    if param.len() <= 0 {
        Err("Empty input".to_string())
    } else {
        Ok(param.to_string())
    }
}

async fn check(param: &str) -> Result<String, String> {
    if param.len() >= 3 {
        Ok(param.to_string())
    } else {
        Err("Too short".to_string())
    }
}

async fn validation(param: &str) -> Result<String, String> {
    println!("{:?}", param.to_string());
    Ok(param.to_string())
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let result1 = receiver("rust").await?;
    let result2 = check(&result1).await?;
    let _result3 = validation(&result2).await?;

    Ok(())
}
