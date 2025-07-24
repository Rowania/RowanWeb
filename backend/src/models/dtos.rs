use chrono::{DateTime, Utc};
use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

// ============= 笔记元数据相关 DTOs =============

#[derive(Debug, Deserialize, Validate)]
pub struct CreateNoteMetadataRequest {
    #[validate(length(min = 1, max = 255))]
    pub slug: String,
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    pub summary: Option<String>,
    pub tags: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateNoteMetadataRequest {
    #[validate(length(min = 1, max = 255))]
    pub slug: Option<String>,
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,
    pub summary: Option<String>,
    pub tags: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NoteMetadataResponse {
    pub id: i32,
    pub file_id: Uuid,
    pub slug: String,
    pub title: String,
    pub summary: Option<String>,
    pub published_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub views: i32,
    pub likes_count: i32,
    pub tags: Option<String>,
    pub category: Option<String>,
}

impl From<crate::models::notes_metadata::Model> for NoteMetadataResponse {
    fn from(note: crate::models::notes_metadata::Model) -> Self {
        Self {
            id: note.id,
            file_id: note.file_id,
            slug: note.slug,
            title: note.title,
            summary: note.summary,
            published_at: note.published_at,
            updated_at: note.updated_at,
            views: note.views,
            likes_count: note.likes_count,
            tags: note.tags,
            category: note.category,
        }
    }
}

// ============= 评论相关 DTOs =============

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCommentRequest {
    #[validate(length(min = 1))]
    pub content: String,
    pub note_metadata_id: i32,
    pub visitor_profile_id: i32,
    pub parent_id: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCommentRequest {
    #[validate(length(min = 1))]
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub id: i32,
    pub note_metadata_id: i32,
    pub visitor_profile_id: i32,
    pub content: String,
    pub parent_id: Option<i32>,
    pub created_at: DateTimeWithTimeZone,
    pub is_approved: bool,
}

impl From<crate::models::comments::Model> for CommentResponse {
    fn from(comment: crate::models::comments::Model) -> Self {
        Self {
            id: comment.id,
            note_metadata_id: comment.note_metadata_id,
            visitor_profile_id: comment.visitor_profile_id,
            content: comment.content,
            parent_id: comment.parent_id,
            created_at: comment.created_at,
            is_approved: comment.is_approved,
        }
    }
}

// ============= 访客配置文件相关 DTOs =============

#[derive(Debug, Deserialize, Validate)]
pub struct CreateVisitorProfileRequest {
    #[validate(length(min = 1, max = 40))]
    pub cookie_id: String,
    #[validate(length(min = 1, max = 13))]
    pub name: String,
    #[validate(length(max = 45))]
    pub ip: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateVisitorProfileRequest {
    #[validate(length(min = 1, max = 13))]
    pub name: Option<String>,
    #[validate(length(max = 45))]
    pub ip: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VisitorProfileResponse {
    pub id: i32,
    pub cookie_id: String,
    pub name: String,
    pub ip: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

impl From<crate::models::visitor_profiles::Model> for VisitorProfileResponse {
    fn from(profile: crate::models::visitor_profiles::Model) -> Self {
        Self {
            id: profile.id,
            cookie_id: profile.cookie_id,
            name: profile.name,
            ip: profile.ip,
            created_at: profile.created_at,
            updated_at: profile.updated_at,
        }
    }
}

// ============= 友链相关 DTOs =============

#[derive(Debug, Deserialize, Validate)]
pub struct CreateFriendsLinkRequest {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    #[validate(url, length(min = 1, max = 255))]
    pub url: String,
    #[validate(length(max = 255))]
    pub description: Option<String>,
    #[validate(url, length(max = 255))]
    pub logo_url: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateFriendsLinkRequest {
    #[validate(length(min = 1, max = 50))]
    pub name: Option<String>,
    #[validate(url, length(min = 1, max = 255))]
    pub url: Option<String>,
    #[validate(length(max = 255))]
    pub description: Option<String>,
    #[validate(url, length(max = 255))]
    pub logo_url: Option<String>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct FriendsLinkResponse {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub sort_order: i32,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

impl From<crate::models::friends_links::Model> for FriendsLinkResponse {
    fn from(link: crate::models::friends_links::Model) -> Self {
        Self {
            id: link.id,
            name: link.name,
            url: link.url,
            description: link.description,
            logo_url: link.logo_url,
            sort_order: link.sort_order,
            created_at: link.created_at,
            updated_at: link.updated_at,
        }
    }
}

// ============= 点赞相关 DTOs =============

#[derive(Debug, Deserialize, Validate)]
pub struct CreateLikeRequest {
    pub note_metadata_id: i32,
    #[validate(length(min = 1, max = 45))]
    pub ip_address: String,
}

#[derive(Debug, Serialize)]
pub struct LikeResponse {
    pub id: i32,
    pub note_metadata_id: i32,
    pub ip_address: String,
}

impl From<crate::models::likes::Model> for LikeResponse {
    fn from(like: crate::models::likes::Model) -> Self {
        Self {
            id: like.id,
            note_metadata_id: like.note_metadata_id,
            ip_address: like.ip_address,
        }
    }
}

// ============= 通用 DTOs =============

#[derive(Debug, Deserialize, Validate)]
pub struct PaginationQuery {
    #[validate(range(min = 1, max = 100))]
    pub page: Option<u64>,
    #[validate(range(min = 1, max = 100))]
    pub page_size: Option<u64>,
}

impl Default for PaginationQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(10),
        }
    }
}
