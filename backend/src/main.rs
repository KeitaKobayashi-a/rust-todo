use axum::{Router, http::HeaderValue, routing::get};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use crate::controller::create_todo_router;
use crate::repository::TodoRepositoryImpl;
use crate::service::TodoUsecase;

mod controller;
mod db;
mod interface;
mod models;
mod repository;
mod service;

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

    let cors =
        CorsLayer::new().allow_origin(["http://localhost:5173".parse::<HeaderValue>().unwrap()]);
    let serve_dir = ServeDir::new("../public");

    let app = Router::new()
        .nest("/api", create_todo_router(todo_service))
        .layer(cors)
        .fallback_service(serve_dir);

    let port: u16 = env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse()?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("🚀 Server running at http://{}", addr);
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
