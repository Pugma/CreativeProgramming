extern crate openapi;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // 最初に初期化をする
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    // HTTP リスナーの定義
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // start listening
    axum::serve(listener, app).await.unwrap();
}
