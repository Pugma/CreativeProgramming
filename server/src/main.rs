use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // 最初に初期化をする
    // 変数のシャドウを使いながらパスを追加
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    // HTTP リスナーの定義
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // start listening
    axum::serve(listener, app).await.unwrap();
}
