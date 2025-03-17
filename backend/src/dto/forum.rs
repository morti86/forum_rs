use core::str;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use crate::models::UserRole;

pub fn validate_roles<T>(v: &Vec<T>) -> Result<(), ValidationError> {
    if v.len() == 0 {
        return Err(ValidationError::new("Section must be allowed for at least one role"));
    }
    Ok(())
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateThreadDto {
    #[validate(length(min = 3, message = "Title too short"))]
    pub title: String,
    #[validate(length(min = 10, message = "A post must contain at least 10 characters"))]
    pub content: String,
    pub section: i64,
    pub hash_tags: Vec<String>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteThreadDto {
    pub thread_id: i64,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdateThreadDto {
    pub thread_id: i64,
    #[validate(length(min = 3, message = "Title too short"))]
    pub title: String,
    #[validate(length(min = 10, message = "A post must contain at least 10 characters"))]
    pub content: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct LockThreadDto {
    pub thread_id: i64,
    pub locked: bool,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct CreateSectionDto {
    #[validate(length(min = 3, message = "Title too short"))]
    name: String,
    description: String,
    #[validate(custom(function = "validate_roles"))]
    allowed_for: Vec<UserRole>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteSectionDto {
    #[validate(range(min=0))]
    s_id: i64,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetChatDto {
    #[validate(range(min=0, max=100))]
    limit: usize,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct PostChatDto {
    #[validate(length(min = 3, message = "Message too short"))]
    content: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeleteChatDto {
    #[validate(range(min=0))]
    post_id: i32,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct GetThreadDto {
    #[validate(range(min=0))]
    pub thread_id: i64,
    #[validate(range(min=1))]
    pub page: i32,
    pub limit: usize,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct UpdatePostDto {
    #[validate(range(min=0))]
    pub post_id: i64,
    pub content: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct DeletePostDto {
    #[validate(range(min=0))]
    pub post_id: i64,
}

//----- Output ------

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetThreadResponseDto {
    pub info: crate::models::Thread,
    pub posts: Vec<crate::models::Post>,
}

#[derive(Serialize, Deserialize)]
pub struct GetSectionsResponseDto {
    pub sections: Vec<crate::models::Section>,
}

#[derive(Serialize, Deserialize)]
pub struct GetSectionResponseDto {
    pub threads: Vec<crate::models::Thread>,
}
