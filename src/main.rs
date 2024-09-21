mod library;
mod model;
mod repository;

use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!(
        "{}",
        env::var("DATABASE_URL").expect("DATABASE URL is not found"),
    );

    let dsn = env::var("DATABASE_URL").expect("DATABASE URL is not found");
    let connection = PgPoolOptions::new()
        .max_connections(5)
        .connect(&dsn)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(connection.clone())
            .service(repository::task_repository::get_task_by_id)
    })
    .bind(("127.0.0.1", 5001))?
    .run()
    .await
}
