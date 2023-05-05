use serde::Serialize;

mod db;
mod models;

#[derive(Debug, Serialize)]
struct Quest {
    quest_id: i32,
    quest_name: String,
    description: String,
    objective: String,
    reward_id: Option<i32>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Create a new quest
    // Retrieve the created quest
    Ok(())
}
