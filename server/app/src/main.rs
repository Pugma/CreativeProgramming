pub mod handler;

extern crate openapi;
use crate::handler::Count;

#[tokio::main]
async fn main() {
    let state = Count(0);
    // 最初に初期化をする
    let app = openapi::server::new(state);

    // HTTP リスナーの定義
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // start listening
    axum::serve(listener, app).await.unwrap();
}
