use anyhow::Ok;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions, MySqlQueryResult};
use sqlx::{FromRow, MySqlPool};

use std::env;
use std::string::String;

// const MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../docs");

use async_sqlx_session::MySqlSessionStore;
use openapi::models::GroupItem;

mod user_groups;
mod user_passwords;
mod user_sessions;

#[derive(Clone)]
pub struct Repository {
    pool: MySqlPool,
    session_store: MySqlSessionStore,
    bcrypt_cost: u32,
}

impl Repository {
    pub async fn setup_db() -> anyhow::Result<Self> {
        // コネクションプールの設定
        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .connect_with(CONFIG.options())
            .await?;

        let session_store =
            MySqlSessionStore::from_client(pool.clone()).with_table_name("user_sessions");

        session_store.migrate().await?;

        Ok(Self {
            pool,
            session_store,
            bcrypt_cost: bcrypt::DEFAULT_COST,
        })
    }

    // pub async fn migrate(&self) -> anyhow::Result<()> {
    //     MIGRATOR.run(&self.pool).await?;
    //     self.session_store.migrate().await?;
    //     Ok(())
    // }
}

impl AsRef<Repository> for Repository {
    fn as_ref(&self) -> &Repository {
        self
    }
}

pub struct DataBaseConfig {
    db_hostname: String,
    db_database: String,
    db_username: String,
    db_password: String,
    db_port: u16,
}

#[derive(FromRow)]
pub struct Password(String);

impl Password {
    fn to_string(&self) -> &str {
        &self.0
    }
}

#[derive(FromRow)]
struct DbGroupItem {
    group_uuid: sqlx::types::Uuid,
    group_name: String,
}

impl DbGroupItem {
    fn to_group_item(&self) -> GroupItem {
        GroupItem::new(self.group_uuid, self.group_name.clone())
    }
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

#[derive(FromRow, Deserialize, Serialize)]
pub struct UserName(String);

impl UserName {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
