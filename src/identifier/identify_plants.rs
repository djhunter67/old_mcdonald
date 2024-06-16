use reqwest;

pub enum VertexError {
    VertexError(String),
}

pub async fn vertex_ask(question: &str) -> Result<String, VertexError> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://www.rust-lang.org")
        .header(reqwest::header::USER_AGENT, "Awesome Rust Program")
        .send()
        .await
        .unwrap();

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());

    let body = serde_json::Value::String(res.text().await.unwrap());
    println!("Body:\n{}", body);

    Ok("".to_string())
}
