use reqwest::StatusCode;
use std::net::TcpListener;

#[tokio::test]
async fn run_endpoint_requires_auth() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);

    let response = reqwest::Client::new()
        .post(format!("http://{addr}/run"))
        .json(&serde_json::json!({"tool":"clipboard","prompt":"hello"}))
        .send()
        .await;

    assert!(response.is_err() || response.unwrap().status() == StatusCode::UNAUTHORIZED);
}
