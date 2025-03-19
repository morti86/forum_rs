use std::sync::Arc;

use axum::{extract::{Query, Path}, middleware, response::IntoResponse, routing::{get, put, post}, Extension, Json, Router};
use validator::Validate;
use crate::AppState;
use crate::{db::user::UserExt,
    models::UserRole,
    dto::user,
    error::{ErrorMessage, HttpError},
    middleware::{role_check, JWTAuthMiddeware},
    utils::password,
};

pub fn user_handler() -> Router {
    let admin_mod_only = middleware::from_fn(|state, req, next| 
                    role_check(state, req, next, vec![UserRole::Admin, UserRole::Mod]) );

    Router::new()
        .route("/me", get(get_me))
        .route("/user/{uuid}", get(get_user_data))
        .route("/list", get(get_users))
        .route("/{user_id}/posts", get(user_posts))
        .route("/{user_id}/threads", get(user_threads))
        .route("/{user_id}/warnings", get(user_warnings))
        .route("/message", post(send_pm))
        .route("/unban", put(unban_user).layer(admin_mod_only.clone()) )
        .route("/warn", put(warn_user).layer(admin_mod_only.clone()) )
        .route("/pms", get(get_pms))
}

pub async fn get_me(
    Extension(_app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>
) -> Result<impl IntoResponse, HttpError> {

    let filtered_user = user::FilterUserDto::filter_user(&user.user);

    let response_data = user::UserResponseDto {
        status: "success".to_string(),
        data: user::UserData {
            user: filtered_user,
        }
    };

    Ok(Json(response_data))
}

pub async fn get_users(
    Query(query_params): Query<user::RequestQueryDto>,
    Extension(app_state): Extension<Arc<AppState>>
) -> Result<impl IntoResponse, HttpError> {
    query_params.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let page = query_params.page.unwrap_or(1);
    let limit = query_params.limit.unwrap_or(10);
    
    let users = app_state.db_client
        .get_users(page as u32, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user_count = app_state.db_client
        .get_user_count()
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = user::UserListResponseDto {
        status: "success".to_string(),
        users: user::FilterUserDto::filter_users(&users),
        results: user_count,
    };

    Ok(Json(response))
}

pub async fn get_user_data(
    Path(uuid) : Path<uuid::Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let user = app_state.db_client.get_user(Some(uuid), None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;
    let response = user::Response {
        status: "success",
        message: user.unwrap_or_else(|| crate::models::User { name: "Deleted User".to_string(), ..Default::default()}).name,
    };

    Ok(Json(response))


}
    
pub async fn update_user_name(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>,
    Json(body): Json<user::NameUpdateDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
       .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = &user.user;

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    app_state.db_client.
        update_user_name(user_id.clone(), &body.name)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    //let filtered_user = user::FilterUserDto::filter_user(&result);

    let response = user::Response {
        status: "success",
        message: "name changed".to_string(),
    };

    Ok(Json(response))
}

pub async fn update_user_role(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>,
    Json(body): Json<user::RoleUpdateDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
        .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = &user.user;

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    app_state.db_client
        .update_user_role(user_id.clone(), body.role)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = user::Response {
        status: "success",
        message: "role changed".to_string(),
    };

    Ok(Json(response))
}

pub async fn update_user_password(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>,
    Json(body): Json<user::UserPasswordUpdateDto>,
) -> Result<impl IntoResponse, HttpError> {
    body.validate()
       .map_err(|e| HttpError::bad_request(e.to_string()))?;

    let user = &user.user;

    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    let result = app_state.db_client
        .get_user(Some(user_id.clone()), None, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let user = result.ok_or(HttpError::unauthorized(ErrorMessage::InvalidToken.to_string()))?;

    let password_match = password::compare(&body.old_password, &user.password)
            .map_err(|e| HttpError::server_error(e.to_string()))?;

    if !password_match {
        return Err(HttpError::bad_request("Old password is incorrect".to_string()));
    }

    let hash_password = password::hash(&body.new_password)
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    app_state.db_client
        .update_user_password(user_id.clone(), hash_password.as_str())
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = user::Response {
        message: "Password updated Successfully".to_string(),
        status: "success",
    };

    Ok(Json(response))
}

pub async fn warn_user(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>,
    Json(body): Json<user::WarnUserDto>,
) -> Result<impl IntoResponse, HttpError> {

    let user = &user.user;
    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    app_state.db_client
        .warn_user(body.uuid, body.comment.as_deref(), user_id, body.banned)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = user::Response {
        message: "User warned".to_string(),
        status: "success",
    };

    Ok(Json(response))
}

pub async fn unban_user(
    Extension(app_state): Extension<Arc<AppState>>,
    Json(body): Json<user::WarnUserDto>,
) -> Result<impl IntoResponse, HttpError> {
    app_state.db_client
        .unban_user(body.uuid)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = user::Response {
        message: "User warned".to_string(),
        status: "success",
    };

    Ok(Json(response))
}

pub async fn user_posts(
    Path(user_id) : Path<uuid::Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let result = app_state.db_client
        .get_user_posts(Some(user_id), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = user::UserPostsResponseDto {
        posts: result,
    };

    Ok(Json(response))
}

pub async fn user_threads(
    Path(user_id) : Path<uuid::Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let threads = app_state.db_client
        .get_user_threads(Some(user_id), None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = user::UserThreadsResponseDto {
        threads,
    };

    Ok(Json(response))
}

pub async fn user_warnings(
    Path(user_id) : Path<uuid::Uuid>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let warnings = app_state.db_client
        .get_user_warnings(user_id, None)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = user::UserWarningsResponseDto {
        warnings,
    };

    Ok(Json(response))
}

pub async fn send_pm(
    Extension(app_state): Extension<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddeware>,
    Json(body): Json<user::SendPmDto>,
) -> Result<impl IntoResponse, HttpError> {
    let user = &user.user;
    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    app_state.db_client.send_pm(user_id, body.recipient_id, body.content.as_str())
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = user::Response {
        message: "Private message sent".to_string(),
        status: "success",
    };

    Ok(Json(response))
}

pub async fn get_pms(
    Path((page,limit)) : Path<(u32,usize)>,
    Extension(user): Extension<JWTAuthMiddeware>,
    Extension(app_state): Extension<Arc<AppState>>,
) -> Result<impl IntoResponse, HttpError> {
    let user = &user.user;
    let user_id = uuid::Uuid::parse_str(&user.id.to_string()).unwrap();

    let pms = app_state.db_client.get_pms(user_id, page, limit)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    let response = user::UserPmsResponseDto { pms };

    Ok(Json(response))
}

