use axum::{Router, routing::get,http::HeaderValue,};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use tower_http::cors::CorsLayer;

use crate::repository::TodoRepositoryImpl;
use crate::controller::create_todo_router;
use crate::service::TodoUsecase;

mod models;
mod interface;
mod db;
mod repository;
mod service;
mod controller;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;

    let todo_repository = TodoRepositoryImpl::new(pool.clone());
    let todo_service = TodoUsecase::new(todo_repository);

    let cors = CorsLayer::new().allow_origin(["http://localhost:5173".parse::<HeaderValue>().unwrap()]);

    let app = Router::new()
        .nest("/api", create_todo_router(todo_service)).layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("🚀 Server running at http://{}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}