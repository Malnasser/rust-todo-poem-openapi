use crate::models::{Todo, User};

pub async fn get_all_users(pool: &sqlx::PgPool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(pool)
        .await
}

pub async fn get_all_todos(pool: &sqlx::PgPool) -> Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as!(Todo, "SELECT * FROM todos")
        .fetch_all(pool)
        .await
}

pub async fn create_todo(
    pool: &sqlx::PgPool,
    user_id: i32,
    title: String,
) -> Result<Todo, sqlx::Error> {
    sqlx::query_as!(
        Todo,
        "INSERT INTO todos (user_id, title, completed) VALUES ($1, $2, false) RETURNING *",
        user_id,
        title
    )
    .fetch_one(pool)
    .await
}

pub async fn get_todo_by_id(pool: &sqlx::PgPool, todo_id: i32) -> Result<Todo, sqlx::Error> {
    sqlx::query_as!(Todo, "SELECT * FROM todos WHERE id = $1", todo_id)
        .fetch_one(pool)
        .await
}

pub async fn update_todo_complete_by_id(pool: &sqlx::PgPool, todo_id: i32) -> Result<Todo, sqlx::Error> {
    sqlx::query_as!(
        Todo,
        "UPDATE todos SET completed = true WHERE id = $1 RETURNING *",
        todo_id
    )
    .fetch_one(pool)
    .await
}
