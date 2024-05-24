pub mod db;
pub mod handler;

extern crate openapi;
use std::thread;

use db::setup_db;

use crate::handler::Count;

#[tokio::main]
async fn main() {
    let db_pool = thread::spawn(|| match setup_db() {
        Ok(a) => {
            println!("Correctly connected to DB!");
            a
        }
        Err(a) => panic!("{a}"),
    })
    .join()
    .expect("Thread panicked");

    let state = Count(db_pool);

    // 最初に初期化をする
    let app = openapi::server::new(state);

    // TCP リスナーの定義
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    // start listening
    axum::serve(listener, app).await.unwrap();
}
