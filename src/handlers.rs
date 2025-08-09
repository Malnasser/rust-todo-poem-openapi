use crate::models::{Todo, User};
use poem::web::Data;
use poem_openapi::{ApiResponse, Object, OpenApi, Tags, param::Path, payload::Json};
use serde::Deserialize;
use sqlx::PgPool;
#[derive(Tags)]
enum ApiTags {
    Users,
    Todo,
}

#[derive(ApiResponse)]
enum UserListResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<User>>),
    #[oai(status = 500)]
    InternalServerError,
}

#[derive(ApiResponse)]
enum TodoListResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Todo>>),
    #[oai(status = 500)]
    InternalServerError,
}

#[derive(ApiResponse)]
enum TodoResponse {
    #[oai(status = 200)]
    Ok(Json<Todo>),
    #[oai(status = 404)]
    NotFound,
    #[oai(status = 500)]
    InternalServerError,
}

#[derive(ApiResponse)]
enum CreateTodoResponse {
    #[oai(status = 201)]
    Created,
    #[oai(status = 500)]
    InternalServerError,
}

#[derive(Debug, Deserialize, Object)]
pub struct CreateTodo {
    pub title: String,
}

pub struct TodoApi;

#[OpenApi]
impl TodoApi {
    #[oai(path = "/users", method = "get", tag = "ApiTags::Users")]
    async fn get_users(&self, pool: Data<&PgPool>) -> UserListResponse {
        match crate::database::get_all_users(pool.0).await {
            Ok(users) => UserListResponse::Ok(Json(users)),
            Err(_) => UserListResponse::InternalServerError,
        }
    }

    #[oai(path = "/todos", method = "get", tag = "ApiTags::Todo")]
    async fn get_todos(&self, pool: Data<&PgPool>) -> TodoListResponse {
        match crate::database::get_all_todos(pool.0).await {
            Ok(todos) => TodoListResponse::Ok(Json(todos)),
            Err(_) => TodoListResponse::InternalServerError,
        }
    }

    #[oai(path = "/todos", method = "post", tag = "ApiTags::Todo")]
    async fn create_todo(&self, pool: Data<&PgPool>, body: Json<CreateTodo>) -> CreateTodoResponse {
        match crate::database::create_todo(pool.0, 1, body.title.clone()).await {
            Ok(_) => CreateTodoResponse::Created,
            Err(_) => CreateTodoResponse::InternalServerError,
        }
    }

    #[oai(path = "/todos/:id", method = "get", tag = "ApiTags::Todo")]
    async fn get_todo(&self, pool: Data<&PgPool>, id: Path<i32>) -> TodoResponse {
        match crate::database::get_todo_by_id(pool.0, id.0).await {
            Ok(todo) => TodoResponse::Ok(Json(todo)),
            Err(sqlx::Error::RowNotFound) => TodoResponse::NotFound,
            Err(_) => TodoResponse::InternalServerError,
        }
    }

    #[oai(path = "/todo/:id/complete-todo", method  = "post", tag = "ApiTags::Todo")]
    async fn complete_todo(&self, pool: Data<&PgPool>, id: Path<i32>) -> TodoResponse {
        match crate::database::update_todo_complete_by_id(pool.0, id.0).await {
            Ok(todo) => TodoResponse::Ok(Json(todo)),
            Err(sqlx::Error::RowNotFound) => TodoResponse::NotFound,
            Err(_) => TodoResponse::InternalServerError,
        }
    }
}
