use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgPool, PgQueryResult, PgRow},
    FromRow,
};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub item_type: String,
}

impl Item {
    pub async fn insert(&self, pool: &PgPool) -> sqlx::Result<PgQueryResult> {
        let result = sqlx::query(
            r#"
            INSERT INTO items (name, description, item_type) VALUES ($1, $2, $3);
        "#,
        )
        .bind(&self.name)
        .bind(&self.description)
        .bind(&self.item_type)
        .execute(pool)
        .await
        .expect("Failed to insert item.");

        Ok(result)
    }

    pub async fn find(pool: &PgPool, item_id: i32) -> Result<Option<Self>, sqlx::Error> {
        let query = "SELECT id, name, description, item_type FROM items WHERE id = $1";
        let row: Option<PgRow> = sqlx::query(query)
            .bind(item_id)
            .fetch_optional(pool)
            .await
            .expect("Failed to execute query.");

        if let Some(row) = row {
            Ok(Some(Item::from_row(&row).unwrap()))
        } else {
            Ok(None)
        }
    }

    async fn find_all(pool: &PgPool) -> sqlx::Result<Vec<Item>> {
        let items = sqlx::query_as::<_, Item>(
            r#"
            SELECT * FROM items
        "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::client::connect;

    #[tokio::test]
    async fn test_insert_find() {
        let pool = connect().await.expect("Failed to connect to Postgres.");

        let item = Item {
            id: 1,
            name: "item1".to_string(),
            description: "item1 description".to_string(),
            item_type: "type1".to_string(),
        };

        let insert_result = item.insert(&pool).await.expect("Failed to insert item.");
        assert_eq!(insert_result.rows_affected(), 1);

        let items = Item::find_all(&pool).await.expect("Failed to find items.");
        assert_ne!(items.len(), 0);

        let retrieved_item = Item::find(&pool, item.id)
            .await
            .expect("Failed to find items.")
            .unwrap_or_else(|| panic!("Failed to find item with id {}.", item.id));

        assert_eq!(retrieved_item.id, item.id);
        assert_eq!(retrieved_item.name, item.name);
        assert_eq!(retrieved_item.description, item.description);
        assert_eq!(retrieved_item.item_type, item.item_type);
    }
}
