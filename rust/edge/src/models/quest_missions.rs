use serde::Serialize;
use sqlx::{
    postgres::{PgQueryResult, PgRow},
    FromRow, PgPool,
};

#[derive(Debug, FromRow, Serialize)]
pub struct QuestsMissions {
    quest_id: i32,
    mission_id: i32,
}
