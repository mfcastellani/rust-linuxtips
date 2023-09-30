use sqlx::{postgres::PgPoolOptions, Postgres};
use std::env;

pub async fn database_pool() -> sqlx::Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("Please, set DATABASE_URL env var");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Unable to connect to database")
}

