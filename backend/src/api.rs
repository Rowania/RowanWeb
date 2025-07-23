pub mod auth_handler;
pub mod comment_handler;
pub mod note_handler;

use axum::{
    routing::{delete, get, post, put},
    Router,
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
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login))
        .route("/me", get(auth_handler::get_current_user))
        .route("/refresh", post(auth_handler::refresh_token))
}

fn note_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(note_handler::list_notes))
        .route("/", post(note_handler::create_note))
        .route("/:id", get(note_handler::get_note))
        .route("/:id", put(note_handler::update_note))
        .route("/:id", delete(note_handler::delete_note))
        .route("/:id/like", post(note_handler::like_note))
        .route("/:id/unlike", delete(note_handler::unlike_note))
}

fn comment_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(comment_handler::create_comment))
        .route("/:id", get(comment_handler::get_comment))
        .route("/:id", put(comment_handler::update_comment))
        .route("/:id", delete(comment_handler::delete_comment))
        .route("/note/:note_id", get(comment_handler::list_comments_by_note))
}
