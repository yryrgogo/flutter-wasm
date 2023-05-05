use serde::Serialize;
use sqlx::{
    postgres::{PgQueryResult, PgRow},
    FromRow, PgPool,
};

#[derive(Debug, FromRow, Serialize)]
pub struct Reward {
    pub id: i32,
    pub item_id: i32,
    pub quantity: i32,
}

impl Reward {
    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let query = r#"
            INSERT INTO rewards (item_id, quantity)
            VALUES ($1, $2)
            ON CONFLICT (id) DO NOTHING;
        "#;
        let result = sqlx::query(query)
            .bind(self.item_id)
            .bind(self.quantity)
            .execute(pool)
            .await
            .expect("Failed to insert reward.");

        Ok(result)
    }

    pub async fn find(pool: &PgPool, reward_id: i32) -> Result<Option<Self>, sqlx::Error> {
        let query = "SELECT id, item_id, quantity FROM rewards WHERE id = $1";
        let row: Option<PgRow> = sqlx::query(query)
            .bind(reward_id)
            .fetch_optional(pool)
            .await
            .expect("Failed to execute query.");

        if let Some(row) = row {
            Ok(Some(Reward::from_row(&row).unwrap()))
        } else {
            Ok(None)
        }
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let query = "SELECT id, item_id, quantity FROM rewards";
        let rows = sqlx::query_as::<_, Self>(query).fetch_all(pool).await?;
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{db::client::connect, models::item::Item};

    #[tokio::test]
    async fn test_insert_find() {
        let pool = connect().await.expect("Failed to connect to Postgres.");

        let item = Item {
            id: 1,
            name: "item1".to_string(),
            description: "item1 description".to_string(),
            item_type: "type1".to_string(),
        };
        item.insert(&pool).await.expect("Failed to insert item.");

        let reward = Reward {
            id: 1,
            item_id: 1,
            quantity: 10,
        };
        let insert_result = reward.insert(&pool).await.unwrap();
        assert_eq!(insert_result.rows_affected(), 1);

        let rewards = Reward::find_all(&pool)
            .await
            .expect("Failed to find rewards.");
        assert_ne!(rewards.len(), 0);

        let retrieved_reward = Reward::find(&pool, reward.id)
            .await
            .expect("Failed to find rewards.")
            .unwrap_or_else(|| panic!("Failed to find reward with id {}.", reward.id));

        assert_eq!(retrieved_reward.id, reward.id);
        assert_eq!(retrieved_reward.item_id, reward.item_id);
        assert_eq!(retrieved_reward.quantity, reward.quantity);

        let retrieved_reward = Reward::find(&pool, reward.id)
            .await
            .expect("Failed to find rewards.")
            .unwrap_or_else(|| panic!("Failed to find reward with id {}.", reward.id));

        assert_eq!(retrieved_reward.id, reward.id);
        assert_eq!(retrieved_reward.item_id, reward.item_id);
        assert_eq!(retrieved_reward.quantity, reward.quantity);
    }
}
