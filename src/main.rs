mod handler;
mod model;
mod route;
mod schema;

use std::sync::Arc;

use dotenv::dotenv;
use route::create_router;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    sqlx::migrate!("./migrations/")
        .run(&pool)
        .await
        .expect("Cannot finish migrations");

    let app = create_router(Arc::new(AppState { db: pool.clone() }));

    let address = format!("0.0.0.0:{}", std::env::var("PORT").expect("Set PORT in env"));

    println!("🚀 Server started successfully on {}", &address);
    let listener = tokio::net::TcpListener::bind(&address).await.expect("Cannot create server");
    axum::serve(listener, app).await.unwrap();
}
