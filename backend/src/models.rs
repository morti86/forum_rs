use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq, Default)]
#[sqlx(type_name = "forum.user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Mod,
    #[default]
    User,
}

impl UserRole {
    pub fn to_str(&self) -> String {
        match self {
            Self::Admin => "admin".to_string(),
            Self::Mod => "mod".to_string(),
            Self::User => "user".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, sqlx::Type, PartialEq)]
#[sqlx(type_name = "forum.user_status", rename_all = "lowercase")]
pub enum UserStatus {
    Active,
    Inactive,
    Banned,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub verified: bool,
    pub verification_token: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub avatar: Option<String>,
    pub facebook: Option<String>,
    pub x_id: Option<String>,
    #[serde(rename = "bannedUntil")]
    pub banned_until: Option<DateTime<Utc>>,
    pub last_online: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ChatPost {
    pub id: i32,
    pub added: DateTime<Utc>,
    pub author: uuid::Uuid,
    pub author_name: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Section {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SectionsAllowed {
    pub id: i32,
    pub section: i64,
    pub role: UserRole,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Thread {
    pub id: i64,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub content: String,
    pub author: uuid::Uuid,
    pub section_id: i64,
    pub locked: bool,
    pub sticky: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ThreadPosts {
    pub thread: Thread,
    pub posts: Vec<Post>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Post {
    pub id: i64,
    pub content: String,
    pub author: Option<uuid::Uuid>,
    pub topic: i64,
    pub comments: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
    pub likes: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Hashtag {
    pub id: i64,
    pub tag: String,
    pub topic: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct UserWarning {
    pub id: i64,
    pub user: uuid::Uuid,
    pub warn_time: DateTime<Utc>,
    pub comment: Option<String>,
    pub warned_by: String,
    pub banned: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PrivateMessage {
    pub id: i64,
    pub author: Option<uuid::Uuid>,
    pub receiver: uuid::Uuid,
    pub content: String,
}
