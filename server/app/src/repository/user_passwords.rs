use super::Repository;
use crate::repository::{MySqlQueryResult, Password};
use sqlx::{query, query_as};

impl Repository {
    pub async fn add_user(
        &self,
        user_name: String,
        password: String,
    ) -> Result<MySqlQueryResult, String> {
        // Insert the task, then obtain the ID of this row
        let password = bcrypt::hash(password, self.bcrypt_cost).unwrap();
        println!("binary: {}, length: {}", password, password.len());

        let result = query("INSERT INTO `users` ( `userName`, `password` ) VALUES ( ?, ? )")
            .bind(user_name)
            .bind(password)
            .execute(&self.pool)
            .await;

        match result {
            Ok(r) => Ok(r),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn check_user(&self, user_name: String, password: String) -> Result<bool, String> {
        let correct_password =
            query_as::<_, Password>("SELECT `password` FROM `users` WHERE `userName`=?")
                .bind(user_name)
                .fetch_one(&self.pool)
                .await;

        match correct_password {
            Ok(correct) => match bcrypt::verify(password, correct.to_string()) {
                Ok(r) => Ok(r),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}
