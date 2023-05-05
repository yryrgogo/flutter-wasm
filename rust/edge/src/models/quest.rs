use serde::Serialize;
use sqlx::{
    postgres::{PgQueryResult, PgRow},
    FromRow, PgPool,
};

#[derive(Debug, FromRow, Serialize)]
pub struct Quest {
    pub id: i32,
    pub name: String,
    pub quest_type: String,
    pub description: String,
    pub objective: String,
    pub reward_id: i32,
}

impl Quest {
    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let query = r#"
            INSERT INTO quests (name, quest_type, description, objective, reward_id)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO NOTHING;
        "#;
        let result = sqlx::query(query)
            .bind(&self.name)
            .bind(&self.quest_type)
            .bind(&self.description)
            .bind(&self.objective)
            .bind(self.reward_id)
            .execute(pool)
            .await
            .expect("Failed to insert quest.");

        Ok(result)
    }

    pub async fn find(pool: &PgPool, quest_id: i32) -> Result<Option<Self>, sqlx::Error> {
        let query = "SELECT id, quest_type, name, description, objective, reward_id FROM quests WHERE id = $1";
        let row: Option<PgRow> = sqlx::query(query)
            .bind(quest_id)
            .fetch_optional(pool)
            .await
            .expect("Failed to execute query.");

        if let Some(row) = row {
            Ok(Some(Quest::from_row(&row).unwrap()))
        } else {
            Ok(None)
        }
    }

    pub async fn find_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let query = "SELECT id, quest_type, name, description, objective, reward_id FROM quests";
        let rows = sqlx::query_as::<_, Self>(query).fetch_all(pool).await?;
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        db::client::connect,
        models::{item::Item, reward::Reward},
    };

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
        reward.insert(&pool).await.unwrap();

        let quest = Quest {
            id: 1,
            name: "Test Quest".to_string(),
            quest_type: "Test".to_string(),
            description: "This is a test quest.".to_string(),
            objective: "Complete the test.".to_string(),
            reward_id: 1,
        };

        let insert_result = quest.insert(&pool).await.unwrap();
        assert_eq!(insert_result.rows_affected(), 1);

        let quests = Quest::find_all(&pool)
            .await
            .expect("Failed to find quests.");
        assert_ne!(quests.len(), 0);

        let retrieved_quest = Quest::find(&pool, quest.id)
            .await
            .expect("Failed to find quest")
            .unwrap_or_else(|| panic!("Failed to find quest with id {}.", quest.id));

        assert_eq!(retrieved_quest.id, quest.id);
        assert_eq!(retrieved_quest.name, quest.name);
        assert_eq!(retrieved_quest.description, quest.description);
        assert_eq!(retrieved_quest.objective, quest.objective);
        assert_eq!(retrieved_quest.reward_id, quest.reward_id);

        let retrieved_quest = Quest::find(&pool, quest.id)
            .await
            .expect("Failed to find quests.")
            .unwrap_or_else(|| panic!("Failed to find quest with id {}.", quest.id));

        assert_eq!(retrieved_quest.id, quest.id);
        assert_eq!(retrieved_quest.name, quest.name);
        assert_eq!(retrieved_quest.description, quest.description);
        assert_eq!(retrieved_quest.objective, quest.objective);
        assert_eq!(retrieved_quest.reward_id, quest.reward_id);
    }
}
