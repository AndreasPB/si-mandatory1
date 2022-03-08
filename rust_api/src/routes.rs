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
        };

        let body = Json(json!({
            "error": err_message,
        }));

        (status, body).into_response()
    }
}
