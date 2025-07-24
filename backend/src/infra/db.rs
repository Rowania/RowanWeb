//ConnectOptions: 用于配置数据库连接的各种选项，比如最大连接数、超时时间等
//Database: 用于建立和管理数据库连接的入口点
//DatabaseConnection: 一个已经建立好的数据库连接的类型
//DbErr: 数据库操作可能遇到的错误类型。
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use std::time::Duration;

/// 数据库连接配置
#[derive(Debug, Clone)]
//为create_db_pool准备的结构体
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    pub max_lifetime_secs: u64,
    pub enable_logging: bool,
}

impl DatabaseConfig {
    /// 从环境变量加载配置，如果环境变量不存在则报错
    pub fn from_env() -> Self {
        Self {
            url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL 环境变量未设置，请在 .env 文件中配置"),
            max_connections: std::env::var("DB_MAX_CONNECTIONS")
                .expect("DB_MAX_CONNECTIONS 环境变量未设置，请在 .env 文件中配置")
                .parse()
                .expect("DB_MAX_CONNECTIONS 必须是有效的数字"),
            min_connections: std::env::var("DB_MIN_CONNECTIONS")
                .expect("DB_MIN_CONNECTIONS 环境变量未设置，请在 .env 文件中配置")
                .parse()
                .expect("DB_MIN_CONNECTIONS 必须是有效的数字"),
            connect_timeout_secs: std::env::var("DB_CONNECT_TIMEOUT_SECS")
                .expect("DB_CONNECT_TIMEOUT_SECS 环境变量未设置，请在 .env 文件中配置")
                .parse()
                .expect("DB_CONNECT_TIMEOUT_SECS 必须是有效的数字"),
            idle_timeout_secs: std::env::var("DB_IDLE_TIMEOUT_SECS")
                .expect("DB_IDLE_TIMEOUT_SECS 环境变量未设置，请在 .env 文件中配置")
                .parse()
                .expect("DB_IDLE_TIMEOUT_SECS 必须是有效的数字"),
            max_lifetime_secs: std::env::var("DB_MAX_LIFETIME_SECS")
                .expect("DB_MAX_LIFETIME_SECS 环境变量未设置，请在 .env 文件中配置")
                .parse()
                .expect("DB_MAX_LIFETIME_SECS 必须是有效的数字"),
            enable_logging: std::env::var("DB_ENABLE_LOGGING")
                .expect("DB_ENABLE_LOGGING 环境变量未设置，请在 .env 文件中配置")
                .parse()
                .expect("DB_ENABLE_LOGGING 必须是有效的布尔值 (true/false)"),
        }
    }

    /// 获取连接超时时间
    pub fn connect_timeout(&self) -> Duration {
        Duration::from_secs(self.connect_timeout_secs)
    }

    /// 获取空闲超时时间
    pub fn idle_timeout(&self) -> Duration {
        Duration::from_secs(self.idle_timeout_secs)
    }

    /// 获取最大生命周期
    pub fn max_lifetime(&self) -> Duration {
        Duration::from_secs(self.max_lifetime_secs)
    }
}

/// 创建数据库连接池(包含适当的连接数、超时设置和日志配置)
///
/// 从环境变量读取所有配置，如果环境变量未设置则报错
pub async fn create_db_pool(_database_url: &str) -> Result<DatabaseConnection, DbErr> {
    let config = DatabaseConfig::from_env();

    //创建一个 ConnectOptions 的新可变实例，它需要一个数据库 URL
    let mut opt = ConnectOptions::new(config.url.clone());
    opt.max_connections(config.max_connections) // 最大连接数(连接池可以同时处理多个请求，超出的请求会排队等待)
        .min_connections(config.min_connections) // 最小连接数(即使没有请求，连接池也会保持最少连接随时待命)
        .connect_timeout(config.connect_timeout()) // 连接超时(如果尝试连接数据库超过指定时间还没有成功，就认为连接失败)
        .idle_timeout(config.idle_timeout()) // 空闲超时(如果一个连接在连接池里空闲了指定时间没有工作，那么这个连接就会被关闭,这可以节省数据库资源)
        .max_lifetime(config.max_lifetime()) // 最大生命周期(一个连接最多可以存活指定时间，防止长时间连接导致内存泄漏、数据库端连接失效等问题)
        .sqlx_logging(config.enable_logging) // 启用 SQL 日志
        .sqlx_logging_level(log::LevelFilter::Debug); //设置 sqlx 日志的详细程度为 Debug 级别，打印更详细的信息

    //使用上面配置好的 ConnectOptions 来连接数据库并 异步 等待创建一个连接池
    Database::connect(opt).await
}

/// 应用状态，只包含数据库连接池
#[derive(Clone)]
pub struct AppState {
    #[allow(dead_code)] // 暂时允许未使用，因为在开发阶段数据库连接暂未完全集成！！！
    pub db: DatabaseConnection,
}

impl AppState {
    /// 创建新的应用状态实例
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

//这段代码在 Web 中应用
//1.应用启动时：你的 Web 应用在启动时，利用 DatabaseConfig 函数加载配置，调用 create_db_pool 函数，传入数据库 URL，然后等待它返回一个 DatabaseConnection（即数据库连接池）
//2.创建应用状态：然后，你会用这个 DatabaseConnection 来创建一个 AppState 实例：let app_state = AppState::new(db_connection_pool);
//3.共享状态：接着，你通常会把这个 app_state 实例通过 Axum 的 Extension 机制，添加到你的 Web 路由中
//4.在请求处理器中使用：当有 Web 请求到来时，你的 Axum 处理器函数就可以通过 Extension<AppState> 来获取到数据库连接池，然后用 SeaORM 进行数据库操作
