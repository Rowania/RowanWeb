use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::{
        HeaderValue, Method,
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    },
    routing::get,
};
use sea_orm::DatabaseConnection;
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod error;
mod infra;
mod models;
mod service;

use crate::{
    api::create_router, config::AppConfig, error::AppResult, infra::db::create_connection,
};

#[tokio::main]
async fn main() -> AppResult<()> {
    // 初始化日志
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "rowan_web_backend=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenvy::dotenv().ok();

    // 加载配置
    let config = AppConfig::from_env()?;
    tracing::info!("Starting Rowan Web Backend on {}", config.server.address());

    // 创建数据库连接
    let db = create_connection(&config.database.url).await?;

    // 创建应用状态
    let app_state = AppState {
        db,
        config: config.clone(),
    };

    // 创建路由
    let app = create_app(app_state);

    // 启动服务器
    let listener = tokio::net::TcpListener::bind(&config.server.address()).await?;
    tracing::info!("Listening on {}", config.server.address());

    axum::serve(listener, app).await?;

    Ok(())
}

fn create_app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let trace = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    Router::new()
        .route("/", get(|| async { "Rowan Web API v1.0" }))
        .nest("/api", create_router())
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 10MB
        .layer(trace)
        .layer(cors)
        .with_state(state)
}

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub config: AppConfig,
}
