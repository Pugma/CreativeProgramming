mod errors;
mod handler;
mod repository;

use anyhow::Ok;

extern crate openapi;
use axum::routing::{get, post};
use axum::Router;
use handler::{
    group_post, login_post, me_get, schedule_group_id_get, schedule_group_id_post,
    schedule_group_id_put, sign_up_post,
};
use tower_http::services::{ServeDir, ServeFile};

pub use errors::{Error, Result};
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
    let app = Router::new()
        .route("/api/group", post(group_post))
        .route("/api/login", post(login_post))
        .route("/api/me", get(me_get))
        .route(
            "/api/schedule/:group_id",
            get(schedule_group_id_get)
                .post(schedule_group_id_post)
                .put(schedule_group_id_put),
        )
        .route("/api/signUp", post(sign_up_post))
        .nest_service("/assets", serve_dir_from_assets)
        .fallback_service(static_dir)
        .with_state(state);

    // TCP リスナーの定義
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // start listening
    axum::serve(listener, app).await?;
    Ok(())
}
