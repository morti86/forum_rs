#![allow(dead_code)]
use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use tracing_subscriber::filter::LevelFilter;
use db::DBClient;
use std::sync::Arc;
use axum::{
    Extension, Router, 
    middleware::from_fn,
    http::{header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method},
};
use axum_server::tls_rustls::RustlsConfig;
use tower_http::{
    trace::TraceLayer,
    cors::CorsLayer,
};
use std::net::SocketAddr;

mod config;
mod models;
mod db;
mod dto;
mod utils;
mod error;
mod mail;
mod handler;
mod middleware;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: config::Config,
    pub db_client: DBClient,
}

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/auth", handler::auth::auth_handler())
        .nest("/users", handler::user::user_handler().layer(from_fn(middleware::auth))) 
        .nest("/forum", handler::forum::forum_handler().layer(from_fn(middleware::auth))) 
        .layer(TraceLayer::new_for_http())
        .layer(Extension(app_state))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(LevelFilter::DEBUG).init();

    dotenv().ok();
    let config = config::Config::init();

    let use_https = config.enable_https;
    let port = if use_https { config.port_https } else { config.port_http };

    let pool = match PgPoolOptions::new()
            .max_connections(10)
            .connect(config.database_url.as_str())
            .await {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::mirror_request())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST,Method::PUT,Method::DELETE]);
    
    let db_client = DBClient::new(pool);

    let app_state = Arc::new(AppState {
        env: config.clone(),
        db_client,
    });

    let a = app_state.clone();
    let app = create_router(a).layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}",port)).await.unwrap();
    if use_https {
        let config = RustlsConfig::from_pem_file("./cert.pem", "./key.pem").await?;
        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        axum_server::bind_rustls(addr, config).serve(app.into_make_service()).await?;
    } else {
        axum::serve(listener, app).await.unwrap();
    }

    Ok(())
}
