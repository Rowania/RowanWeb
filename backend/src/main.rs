use axum::{Extension, Json, routing::get};
use std::sync::Arc;

//mod api;
mod infra;
mod schema;

use infra::db::{AppState, create_db_pool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    env_logger::init();

    // 加载环境变量
    dotenvy::dotenv().expect("无法加载 .env 文件，请确保 .env 文件存在且格式正确");

    // 获取数据库URL，必须在 .env 文件中配置
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL 环境变量未设置，请在 .env 文件中配置");

    // 创建数据库连接池
    log::info!("🔗 正在连接数据库: {database_url}......");
    let db = create_db_pool(&database_url).await?;
    log::info!("✅ 数据库连接池创建成功！");

    // 创建应用状态
    let app_state = AppState::new(db);

    //let annotated_router = api::create_api_router();
    //let api_docs = Arc::new(annotated_router.annotations().clone());
    //let app_router = annotated_router.build();
    //let api_docs_for_handler = Arc::clone(&api_docs);

    // let app = app_router
    //     .route(
    //         "/api/docs",
    //         get(move || {
    //             let docs = Arc::clone(&api_docs_for_handler);
    //             async move { Json((*docs).clone()) }
    //         }),
    //     )
    //     .layer(Extension(app_state)); // 添加应用状态作为扩展

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .expect("Failed to bind to address 127.0.0.1:5000. Is the port already in use?");

    println!("🚀 服务器已启动!");
    println!("📍 服务地址: http://127.0.0.1:5000");
    println!("📝 API 端点:");
    // for endpoint in api_docs.iter() {
    //     println!(
    //         "   {:?} {} - {}",
    //         endpoint.method, endpoint.path, endpoint.description
    //     );
    // }
    println!("📚 API 文档: http://127.0.0.1:5000/api/docs");
    println!("💾 数据库: {database_url}");
    println!();

    // axum::serve(listener, app)
    //     .await
    //     .expect("Axum server failed to start or encountered a fatal error");

    Ok(())
}
