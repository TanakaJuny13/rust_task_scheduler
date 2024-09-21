use actix_web::{
    get, post,
    web::{self, Json},
    HttpResponse, Responder,
};
use sqlx::{Pool, Postgres};

use crate::model::task_model::{self, Task, TaskError};

#[get("/task/{id}")]
pub async fn get_task_by_id(
    connection: web::Data<Pool<Postgres>>,
    id: web::Path<i64>,
) -> impl Responder {
    let task_id = id.into_inner();
    let task_result = sqlx::query_as!(Task, "SELECT * FROM task WHERE id = $1", task_id)
        .fetch_one(connection.get_ref())
        .await;

    match task_result {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

#[post("/task")]
pub async fn insert_task(
    connection: web::Data<Pool<Postgres>>,
    data: Json<Task>,
) -> impl Responder {
    let result = sqlx::query!("INSERT INTO task (details) VALUES ($1)", data.details)
        .fetch_one(connection.get_ref())
        .await;

    match result {
        Ok(task) => HttpResponse::Ok().json(),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}
