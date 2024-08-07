use super::{Repository, UserName};
use crate::repository::{DbGroupItem, GroupItem};
use sqlx::{query, query_as};
use uuid::Uuid;

#[allow(unused)]
impl Repository {
    pub async fn get_groups_by_user(&self, user_name: UserName) -> Result<Vec<GroupItem>, String> {
        let user_name: String = user_name.get_string();

        // TODO: テーブル結合を使いながらUUIDを返せるようなsqlを書く
        let groups = query_as::<_, DbGroupItem>(
            "
            SELECT `groupUuid`, `groupName`
            FROM
                `users`
            INNER JOIN `userGroup` ON users.userId = userGroup.userId
            INNER JOIN `groups` ON userGroup.groupId = groups.groupId
            WHERE users.userName = ?
            ",
        )
        .bind(user_name)
        .fetch_all(&self.pool)
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

    pub async fn create_group(&self, group_name: String, owner_name: String) -> sqlx::Result<()> {
        let uuid = Uuid::now_v7();
        let _request = query(
            "
            INSERT INTO `groups`
            ( `groupUuid`, `groupName`, `ownerName` )
            VALUES ( ?, ?, ? )
            ",
        )
        .bind(uuid)
        .bind(group_name)
        .bind(owner_name)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
