mod api;
mod application;
use std::env;
use dotenv::dotenv;
use std::net::SocketAddr;
use sqlx::{postgres::PgPoolOptions};
use api::main_router::create_main_router;


#[tokio::main]
async fn main() {

    dotenv().ok();
    
    let app = create_main_router();
    let port:u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a valud u16");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    let url = env::var("DB_URL")
        .expect("DB_URL must be set");
    let _pool = PgPoolOptions::new()
        .max_connections(5) //TODO: Make configurable
        .connect(&url)
        .await
        .expect("Failed to create pool.");

}


