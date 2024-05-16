use once_cell::sync::Lazy;

use anyhow;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::env;
use std::string::String;

pub struct DataBaseConfig {
    db_hostname: String,
    db_database: String,
    db_username: String,
    db_password: String,
    db_port: String,
}

impl DataBaseConfig {
    fn db_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.db_username, self.db_password, self.db_hostname, self.db_port, self.db_database
        )
    }
}

static CONFIG: Lazy<DataBaseConfig> = Lazy::new(|| DataBaseConfig {
    db_hostname: env::var("DB_HOSTNAME").unwrap(),
    db_port: env::var("DB_PORT").unwrap(),
    db_username: env::var("DB_USERNAME").unwrap(),
    db_password: env::var("DB_PASSWORD").unwrap(),
    db_database: env::var("DB_DATABASE").unwrap(),
});

#[tokio::main(flavor = "current_thread")]
pub async fn setup_db() -> Result<Pool<MySql>, String> {
    // コネクションプールの設定
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(CONFIG.db_url().as_str())
        .await
    {
        Ok(a) => a,
        Err(_) => return Err("".to_string()),
    };

    Ok(pool)
}

pub async fn add_todo(pool: Pool<MySql>, _: String) -> anyhow::Result<u64> {
    // Insert the task, then obtain the ID of this row
    let todo_id = sqlx::query(
        r#"
INSERT INTO todos ( description )
VALUES ( ? )
        "#,
    )
    .execute(&pool)
    .await?
    .last_insert_id();

    Ok(todo_id)
}
