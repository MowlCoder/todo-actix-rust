use crate::models::{AppState, Status, CreateTodoList, CreateTodoItem, ResultResponse};
use crate::db;
use crate::errors::{AppError};
use deadpool_postgres::{Pool, Client};
use actix_web::{web, Responder, HttpResponse};
use slog::{o, crit, Logger};

pub async fn get_client(pool: Pool, logger: Logger) -> Result<Client, AppError> {
    pool.get().await
        .map_err(|err| {
            let sublog = logger.new(o!("cause" => err.to_string()));
            crit!(sublog, "Error creating client");

            AppError::db_error(err)
        })
}

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "Ok".to_string() })
}

pub async fn get_todos(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let logger = state.logger.new(o!("handler" => "get_todos"));

    let client: Client = get_client(state.pool.clone(), logger.clone()).await?;

    let result = db::get_todos(&client).await;

    result.map(|todos| HttpResponse::Ok().json(todos))
}

pub async fn get_items(state: web::Data<AppState>, path: web::Path<(i32,)>) -> Result<impl Responder, AppError> {
    let client: Client = state.pool.get()
        .await.map_err(AppError::db_error)?;

    let result = db::get_items(&client, path.0).await;

    result.map(|items| HttpResponse::Ok().json(items))
}

pub async fn create_todo(state: web::Data<AppState>, body: web::Json<CreateTodoList>) -> Result<impl Responder, AppError> {
    let client: Client = state.pool.get()
        .await.map_err(AppError::db_error)?;

    let result = db::create_todo(&client, body.title.clone()).await;

    result.map(|todo| HttpResponse::Ok().json(todo))
}

pub async fn create_todo_item(state: web::Data<AppState>, body: web::Json<CreateTodoItem>) -> Result<impl Responder, AppError> {
    let client: Client = state.pool.get()
        .await.map_err(AppError::db_error)?;

    let result = db::create_todo_item(&client, body.title.clone(), body.list_id).await;

    result.map(|todo_item| HttpResponse::Ok().json(todo_item))
}

pub async fn check_item(state: web::Data<AppState>, path: web::Path<(i32,i32)>) -> Result<impl Responder, AppError> {
    let client: Client = state.pool.get()
        .await.map_err(AppError::db_error)?;

    let result = db::check_item(&client, path.0, path.1).await;

    result.map(|updated| HttpResponse::Ok().json(ResultResponse{success: updated}))
}