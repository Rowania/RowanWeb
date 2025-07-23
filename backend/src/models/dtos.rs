use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// ============= 用户相关 DTOs =============

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<crate::models::user::Model> for UserResponse {
    fn from(user: crate::models::user::Model) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            bio: user.bio,
            created_at: user.created_at,
        }
    }
}

// ============= 笔记相关 DTOs =============

#[derive(Debug, Deserialize, Validate)]
pub struct CreateNoteRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
    pub summary: Option<String>,
    pub status: Option<String>,
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateNoteRequest {
    #[validate(length(min = 1, max = 200))]
    pub title: Option<String>,
    #[validate(length(min = 1))]
    pub content: Option<String>,
    pub summary: Option<String>,
    pub status: Option<String>,
    pub tags: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct NoteResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub author_id: Uuid,
    pub status: String,
    pub tags: Option<serde_json::Value>,
    pub views_count: i32,
    pub likes_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<crate::models::note::Model> for NoteResponse {
    fn from(note: crate::models::note::Model) -> Self {
        Self {
            id: note.id,
            title: note.title,
            content: note.content,
            summary: note.summary,
            author_id: note.author_id,
            status: note.status,
            tags: note.tags,
            views_count: note.views_count,
            likes_count: note.likes_count,
            created_at: note.created_at,
            updated_at: note.updated_at,
        }
    }
}

// ============= 评论相关 DTOs =============

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(min = 1))]
    pub content: String,
    pub note_id: Uuid,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCommentRequest {
    #[validate(length(min = 1))]
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: Uuid,
    pub content: String,
    pub note_id: Uuid,
    pub author_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<crate::models::comment::Model> for CommentResponse {
    fn from(comment: crate::models::comment::Model) -> Self {
        Self {
            id: comment.id,
            content: comment.content,
            note_id: comment.note_id,
            author_id: comment.author_id,
            parent_id: comment.parent_id,
            created_at: comment.created_at,
            updated_at: comment.updated_at,
        }
    }
}

// ============= 通用 DTOs =============

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_per_page")]
    pub per_page: u64,
}

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u64 {
    20
}
