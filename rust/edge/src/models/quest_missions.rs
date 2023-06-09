use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgQueryResult},
    FromRow, PgPool,
};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct QuestMissions {
    quest_id: i32,
    mission_id: i32,
}

impl QuestMissions {
    pub async fn insert(&self, pool: &PgPool) -> Result<PgQueryResult, sqlx::Error> {
        let query = r#"
            INSERT INTO quests_missions (quest_id, mission_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING;
        "#;
        let result = sqlx::query(query)
            .bind(self.quest_id)
            .bind(self.mission_id)
            .execute(pool)
            .await
            .expect("Failed to insert quest mission.");

        Ok(result)
    }

    pub async fn find_all_by_quest(pool: &PgPool, quest_id: i32) -> Result<Vec<Self>, sqlx::Error> {
        let query = "SELECT quest_id, mission_id FROM quests_missions WHERE quest_id = $1 ORDER BY mission_id ASC";
        let rows = sqlx::query_as::<_, Self>(query)
            .bind(quest_id)
            .fetch_all(pool)
            .await?;
        Ok(rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        db::client::connect,
        models::{item::Item, mission::Mission, quest::Quest, reward::Reward},
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
        reward
            .insert(&pool)
            .await
            .expect("Failed to insert reward.");

        let quest = Quest {
            id: 1,
            name: "Test Quest".to_string(),
            quest_type: "Test".to_string(),
            description: "This is a test quest.".to_string(),
            objective: "Complete the test.".to_string(),
            reward_id: 1,
        };
        quest.insert(&pool).await.expect("Failed to insert quest.");

        let mission1 = Mission {
            id: 1, // it will be autogenerated by the SERIAL PRIMARY KEY
            name: String::from("Mission Test"),
            description: String::from("Test the mission struct"),
            objective: String::from("Check if the mission can be stored and retrieved"),
            mission_type: String::from("test"),
            requirements: serde_json::json!({"test": "test"}),
        };
        mission1
            .insert(&pool)
            .await
            .expect("Failed to insert mission.");

        let mission2 = Mission {
            id: 1, // it will be autogenerated by the SERIAL PRIMARY KEY
            name: String::from("Mission Test"),
            description: String::from("Test the mission struct"),
            objective: String::from("Check if the mission can be stored and retrieved"),
            mission_type: String::from("test"),
            requirements: serde_json::json!({"test": "test"}),
        };
        mission2
            .insert(&pool)
            .await
            .expect("Failed to insert mission.");

        let quest_mission1 = QuestMissions {
            quest_id: 1,
            mission_id: 1,
        };
        let insert_result = quest_mission1.insert(&pool).await.unwrap();
        assert_eq!(insert_result.rows_affected(), 1);

        let quest_mission2 = QuestMissions {
            quest_id: 1,
            mission_id: 2,
        };
        let insert_result = quest_mission2.insert(&pool).await.unwrap();
        assert_eq!(insert_result.rows_affected(), 1);

        let quest_missions = QuestMissions::find_all_by_quest(&pool, quest_mission1.quest_id)
            .await
            .expect("Failed to find missions for quest.");

        assert_ne!(quest_missions.len(), 0);
        assert_eq!(quest_missions[0].quest_id, quest_mission1.quest_id);
        assert_eq!(quest_missions[0].mission_id, quest_mission1.mission_id);
        assert_eq!(quest_missions[1].quest_id, quest_mission2.quest_id);
        assert_eq!(quest_missions[1].mission_id, quest_mission2.mission_id);
    }
}
