pub mod auth_handler;
pub mod comment_handler;
pub mod note_handler;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::AppState;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth_routes())
        .nest("/notes", note_routes())
        .nest("/comments", comment_routes())
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(auth_handler::get_current_user))
        // SSH 认证将在中间件层处理，不需要注册/登录路由
}

fn note_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(note_handler::list_notes))
        .route("/", post(note_handler::create_note)) // 仅管理员可访问
        .route("/:id", get(note_handler::get_note))
        .route("/:id", put(note_handler::update_note)) // 仅管理员可访问
        .route("/:id", delete(note_handler::delete_note)) // 仅管理员可访问
        .route("/:id/like", post(note_handler::like_note))
        .route("/:id/unlike", delete(note_handler::unlike_note))
}

fn comment_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(comment_handler::create_comment))
        .route("/:id", get(comment_handler::get_comment))
        .route("/:id", put(comment_handler::update_comment))
        .route("/:id", delete(comment_handler::delete_comment))
        .route(
            "/note/:note_id",
            get(comment_handler::list_comments_by_note),
        )
}
