use serde::Serialize;
use sqlx::FromRow;


#[derive(Debug, FromRow, Serialize)]
pub struct MissionStep {
    id: i32,
    step_order: i32,
    mission_id: i32,
    status: String,
    description: String,
    step_image_url: Option<String>,
}
