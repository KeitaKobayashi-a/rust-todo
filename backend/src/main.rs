use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

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
async fn main() {}