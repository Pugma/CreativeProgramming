use once_cell::sync::Lazy;

use sqlx::mysql::{MySqlConnectOptions, MySqlQueryResult};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::env;
use std::string::String;

pub struct DataBaseConfig {
    db_hostname: String,
    db_database: String,
    db_username: String,
    db_password: String,
    db_port: u16,
}

impl DataBaseConfig {
    fn options(&self) -> MySqlConnectOptions {
        MySqlConnectOptions::new()
            .host(&self.db_hostname)
            .port(self.db_port)
            .username(&self.db_username)
            .password(&self.db_password)
            .database(&self.db_database)
    }
}

static CONFIG: Lazy<DataBaseConfig> = Lazy::new(|| DataBaseConfig {
    db_hostname: env::var("DB_HOSTNAME").unwrap(),
    db_port: env::var("DB_PORT").unwrap().parse().unwrap(),
    db_username: env::var("DB_USERNAME").unwrap(),
    db_password: env::var("DB_PASSWORD").unwrap(),
    db_database: env::var("DB_DATABASE").unwrap(),
});

#[tokio::main(flavor = "current_thread")]
pub async fn setup_db() -> Result<Pool<MySql>, String> {
    // コネクションプールの設定
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect_with(CONFIG.options())
        .await;

    match pool {
        Ok(p) => return Ok(p),
        Err(e) => return Err(e.to_string()),
    };
}

pub async fn add_user(
    pool: Pool<MySql>,
    user_name: String,
    password: String,
) -> Result<MySqlQueryResult, String> {
    // Insert the task, then obtain the ID of this row
    let password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
    // let password = format!("{:x}", hashed_password);
    let result = sqlx::query("INSERT INTO `users` ( `userName`, `password` )VALUES ( ?, ? )")
        .bind(user_name)
        .bind(password)
        .execute(&pool)
        .await;

    match result {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }
}
