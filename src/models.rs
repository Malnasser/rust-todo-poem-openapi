use chrono::NaiveDateTime;
use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Object)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub email: String,
    #[oai(skip_serializing_if_is_none)]
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Object)]
pub struct Todo {
    pub id: i32,
    pub user_id: Option<i32>,
    pub title: String,
    pub completed: bool,
    #[oai(skip_serializing_if_is_none)]
    pub created_at: Option<NaiveDateTime>,
}
