use anyhow::Result;
use sqlx::mysql::MySqlQueryResult;

use super::Repository;

#[allow(unused)]
impl Repository {
    pub async fn register_schedule(self) -> Result<MySqlQueryResult> {
        unimplemented!()
    }

    pub async fn get_schedule(self) -> Result<MySqlQueryResult> {
        unimplemented!()
    }

    pub async fn edit_schedule(self) -> Result<MySqlQueryResult> {
        unimplemented!()
    }
}
