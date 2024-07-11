mod handler;
mod repository;

use anyhow::Ok;

extern crate openapi;
use tower_http::services::{ServeDir, ServeFile};

use repository::Repository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // DB 初期化
    let state = Repository::setup_db().await?;
    // state.migrate().await?;

    // 静的ファイル配信設定
    let static_dir = ServeFile::new("/dist/index.html");
    let serve_dir_from_assets = ServeDir::new("/dist/assets/");

    // API 初期化
    let app = openapi::server::new(state)
        .nest_service("/assets", serve_dir_from_assets)
        .fallback_service(static_dir);

    // TCP リスナーの定義
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // start listening
    axum::serve(listener, app).await?;
    Ok(())
}
