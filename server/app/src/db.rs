use once_cell::sync::Lazy;

use sqlx::{
    mysql::{MySqlConnectOptions, MySqlPoolOptions, MySqlQueryResult},
    query, query_as, FromRow, MySql, Pool,
};
use std::env;
use std::string::String;

use openapi::models::GroupItem;

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
    println!("binary: {}, length: {}", password, password.len());

    // let password = format!("{:x}", hashed_password);
    let result = query("INSERT INTO `users` ( `userName`, `password` ) VALUES ( ?, ? )")
        .bind(user_name)
        .bind(password)
        .execute(&pool)
        .await;

    match result {
        Ok(r) => Ok(r),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn check_user(
    pool: Pool<MySql>,
    user_name: String,
    password: String,
) -> Result<bool, String> {
    let correct_password =
        query_as::<_, Password>("SELECT `password` FROM `users` WHERE `userName`=?")
            .bind(user_name)
            .fetch_one(&pool)
            .await;

    match correct_password {
        Ok(correct) => match bcrypt::verify(password, correct.to_string()) {
            Ok(r) => Ok(r),
            Err(e) => Err(e.to_string()),
        },
        Err(e) => Err(e.to_string()),
    }
}

pub async fn get_groups_by_user(
    pool: Pool<MySql>,
    user_name: String,
) -> Result<Vec<GroupItem>, String> {
    // TODO: テーブル結合を使いながらUUIDを返せるようなsqlを書く
    let groups = query_as::<_, DbGroupItem>(
        "SELECT `groupUuid`, `groupName` FROM `groups` JOIN `userGroup`",
    )
    .bind(user_name)
    .fetch_all(&pool)
    .await;

    match groups {
        Ok(groups) => {
            let mut result_groups: Vec<GroupItem> = Vec::<GroupItem>::new();
            for group in groups {
                result_groups.push(group.to_group_item().clone());
            }
            Ok(result_groups)
        }
        Err(aaa) => Err(aaa.to_string()),
    }
}
