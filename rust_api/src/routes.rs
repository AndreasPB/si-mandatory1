use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use mongodb::bson::doc;
use serde_json::json;

use crate::document::User;

pub enum UserError {
    NotFound,
    NotCreated,
}

// Handler for `GET /find-user/:name`
pub async fn find_user(
    Path(name): Path<String>,
    Extension(db_client): Extension<mongodb::Database>,
) -> Result<Json<User>, UserError> {
    let users = db_client.collection::<User>("users");

    let filter = doc! { "name": name };
    let user = match users.find_one(filter, None).await? {
        Some(user) => user,
        None => return Err(UserError::NotFound),
    };

    Ok(user.into())
}

// Handler for POST /add-user/
pub async fn add_user(
    Json(user): Json<User>,
    Extension(db_client): Extension<mongodb::Database>,
) -> Result<Json<User>, UserError> {
    db_client
        .collection::<User>("users")
        .insert_one(user.to_owned(), None)
        .await
        .map_err(|_| UserError::NotCreated)?;

    Ok(user.into())
}

// TODO: should probably not just return not found
impl From<mongodb::error::Error> for UserError {
    fn from(_error: mongodb::error::Error) -> Self {
        UserError::NotFound
    }
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let (status, err_message) = match self {
            UserError::NotFound => (StatusCode::NOT_FOUND, "User not found"),
            UserError::NotCreated => (StatusCode::UNPROCESSABLE_ENTITY, "User not created"),
        };

        let body = Json(json!({
            "error": err_message,
        }));

        (status, body).into_response()
    }
}
