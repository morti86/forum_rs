use std::sync::Arc;

use axum::{extract::Path, middleware::from_fn, response::IntoResponse, routing::{get, put, post, delete}, Extension, Json, Router};
use validator::Validate;
use crate::AppState;
use crate::{db::forum::ForumExt,
    models::UserRole,
    dto::forum,
    error::HttpError,
    middleware::{role_check, JWTAuthMiddeware},
};

pub fn forum_handler() -> Router {
    let admin_mod_only = from_fn(|state, req, next| 
        role_check(state, req, next, vec![UserRole::Admin, UserRole::Mod]) );

    Router::new()
        .route("/", get(get_sections))
        .route("/section/{section_id}", get(get_threads))
        .route("/threads", post(create_thread))
        .route("/threads", delete(delete_thread).layer(admin_mod_only.clone()) )
        .route("/threads", put(update_thread))
        .route("/threads/{thread_id}", get(get_thread))
        .route("/threads/lock", put(lock_thread).layer(admin_mod_only.clone()) )
        .route("/post", put(update_post))
        .route("/post", delete(delete_post))
}

pub async fn create_thread(Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>,
    Json(body): Json<forum::CreateThreadDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;
    let user = &user.user;
    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();
    let hash_tags = body.hash_tags;
    app_state.db_client.create_thread(user_id, body.section, body.title.as_str(), body.content.as_str(), &hash_tags )
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = forum::Response {
        status: "success",
        message: "thread created".to_string(),
    };

    Ok(Json(response))

}

pub async fn delete_thread(Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<forum::DeleteThreadDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;
    app_state.db_client.delete_thread(body.thread_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = forum::Response {
        status: "success",
        message: "thread deleted".to_string(),
    };

    Ok(Json(response))
}


pub async fn update_thread(Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>,
    Json(body): Json<forum::UpdateThreadDto>,
) -> Result<impl IntoResponse, HttpError> {

    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;
    let user = &user.user;
    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();
    let user_role = user.role;

    if user_role != UserRole::Admin && 
        user_role != UserRole::Mod && 
        app_state.db_client.get_thread_author(body.thread_id).await.map_err(|e| HttpError::server_error(e.to_string()))? != user_id {
            return Err(HttpError::unauthorized("Not authorized to edit this thread"));
    }

    app_state.db_client.update_thread(body.thread_id, body.title.as_str(), body.content.as_str() )
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = forum::Response {
        status: "success",
        message: "thread updated".to_string(),
    };

    Ok(Json(response))

}

pub async fn get_thread(
    Path((thread_id,page,limit)) : Path<(i64,i32,usize)>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {

    let thread = app_state.db_client.get_thread_info(thread_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let posts = app_state.db_client.get_thread(thread_id,page,limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = forum::GetThreadResponseDto {
        info: thread,
        posts,
    };

    Ok(Json(response))
}

pub async fn lock_thread(Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<forum::LockThreadDto>,
) -> Result<impl IntoResponse, HttpError> {
    app_state.db_client.lock_thread(body.thread_id, body.locked)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = forum::Response {
        status: "success",
        message: "thread updated".to_string(),
    };

    Ok(Json(response))
}

pub async fn get_sections(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>,
) -> Result<impl IntoResponse, HttpError> {
    let user = &user.user;
    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    let sections = app_state.db_client.get_sections(user_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = forum::GetSectionsResponseDto { sections };

    Ok(Json(response))
}

pub async fn get_threads(
    Path((section_id,page,limit)) : Path<(i64,i32,usize)>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {

    let threads = app_state.db_client.get_section(section_id, page, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = forum::GetSectionResponseDto { threads };

    Ok(Json(response))
}

pub async fn update_post(Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>,
    Json(body): Json<forum::UpdatePostDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;
    let user = &user.user;
    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();
    let user_role = user.role;

    if user_role != UserRole::Admin && 
        user_role != UserRole::Mod && 
        app_state.db_client.get_post_author(body.post_id).await.map_err(|e| HttpError::server_error(e.to_string()))?.unwrap_or_default() != user_id {
            return Err(HttpError::unauthorized("Not authorized to edit this thread"));
    }

    app_state.db_client.update_post(body.post_id, body.content.as_str())
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = forum::Response {
        status: "success",
        message: "post updated".to_string(),
    };

    Ok(Json(response))

}

pub async fn delete_post(Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>,
    Json(body): Json<forum::UpdatePostDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate().map_err(|e| HttpError::bad_request(e.to_string()))?;
    let user = &user.user;
    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();
    let user_role = user.role;

    if user_role != UserRole::Admin && 
        user_role != UserRole::Mod && 
        app_state.db_client.get_post_author(body.post_id).await.map_err(|e| HttpError::server_error(e.to_string()))?.unwrap_or_default() != user_id {
            return Err(HttpError::unauthorized("Not authorized to edit this thread"));
    } 

    if app_state.db_client.posts_since(body.post_id).await.map_err(|e| HttpError::server_error(e.to_string()))? != 0 {
        return Err(HttpError::answered_post_deletion("Cannot delete posts that have answers"));
    }

    app_state.db_client.delete_post(body.post_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = forum::Response {
        status: "success",
        message: "post updated".to_string(),
    };

    Ok(Json(response))

}

