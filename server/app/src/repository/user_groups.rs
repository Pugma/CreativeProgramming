use super::Repository;
use crate::repository::{DbGroupItem, GroupItem};
use sqlx::query_as;

impl Repository {
    pub async fn get_groups_by_user(&self, user_name: String) -> Result<Vec<GroupItem>, String> {
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
}
