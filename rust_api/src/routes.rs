// futures_util provides common utilities and extension traits
// for async programming
use futures_util::StreamExt;

use axum::{
    extract::{Extension, Form, Path},
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
    IO,
}

// Handler for `GET /user`
pub async fn get_all_users(
    Extension(db_client): Extension<mongodb::Database>,
) -> Result<Json<Vec<User>>, UserError> {
    let users = db_client.collection::<User>("users");

    let mut cursor = users.find(None, None).await?;
    let mut retrieved_users: Vec<User> = Vec::new();
    while let Some(user) = cursor.next().await {
        retrieved_users.push(user?);
    }

    Ok(retrieved_users.into())
}

// Handler for `GET /user/:name`
pub async fn get_user(
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

// Handler for POST /user
// https://docs.rs/axum/latest/axum/extract/struct.Form.html
pub async fn post_user(
    form: Form<User>,
    Extension(db_client): Extension<mongodb::Database>,
) -> Result<Json<User>, UserError> {
    let user = form.0;
    db_client
        .collection::<User>("users")
        .insert_one(user.to_owned(), None)
        .await
        .map_err(|_| UserError::NotCreated)?;

    Ok(user.into())
}

impl From<mongodb::error::Error> for UserError {
    fn from(error: mongodb::error::Error) -> Self {
        match *error.kind {
            mongodb::error::ErrorKind::Write(_) => UserError::NotCreated,
            _ => UserError::IO,
        }
    }
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let (status, err_message) = match self {
            UserError::NotFound => (StatusCode::NOT_FOUND, "User not found"),
            UserError::NotCreated => (StatusCode::UNPROCESSABLE_ENTITY, "User not created"),
            UserError::IO => (StatusCode::REQUEST_TIMEOUT, "Request timed out"),
        };

        let body = Json(json!({
            "error": err_message,
        }));

        (status, body).into_response()
    }
}
