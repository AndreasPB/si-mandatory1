use rand::Rng;

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

use crate::document::{UserBase, UserLogin, UserRegister};

pub enum UserError {
    NotFound,
    NotCreated,
    IO,
}

// --------- UTIL --------------
fn generate_token() -> String {
    const TOKEN_CHAR_SET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = rand::thread_rng();
    (0..4)
        .map(|_| TOKEN_CHAR_SET[rng.gen_range(0..TOKEN_CHAR_SET.len())] as char)
        .collect::<String>()
}

// Handler for `GET /user`
pub async fn get_all_users(
    Extension(db_client): Extension<mongodb::Database>,
) -> Result<Json<Vec<UserBase>>, UserError> {
    let users = db_client.collection::<UserBase>("User");

    let mut cursor = users.find(None, None).await?;
    let mut retrieved_users: Vec<UserBase> = Vec::new();
    while let Some(user) = cursor.next().await {
        retrieved_users.push(user?);
    }

    Ok(retrieved_users.into())
}

// Handler for `GET /user/:name`
pub async fn get_user(
    Path(name): Path<String>,
    Extension(db_client): Extension<mongodb::Database>,
) -> Result<Json<UserBase>, UserError> {
    let users = db_client.collection::<UserBase>("User");

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
    form: Form<UserRegister>,
    Extension(db_client): Extension<mongodb::Database>,
) -> Result<Json<UserBase>, UserError> {
    let user = form.0;

    let token = generate_token();
    let user = UserBase {
        name: user.name,
        phone: user.phone,
        token: token.to_string(),
    };
    db_client
        .collection::<UserBase>("User")
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
