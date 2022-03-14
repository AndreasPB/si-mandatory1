use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use mongodb::bson::doc;
use serde_json::{json, Value};

use crate::document::{JwtClaim, Message, UserBase, UserLogin, UserRegister};

const SECRET: &str = "6horse9";

#[derive(Debug)]
pub enum JwtError {
    InvalidToken,
    InvalidSignature,
    ExpiredSignature,
}

pub enum UserError {
    AlreadyExists,
    NotFound,
    NotCreated,
    PasswordMismatch,
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

// https://docs.rs/axum/latest/axum/extract/struct.Form.html
pub async fn post_user(
    form: Form<UserRegister>,
    Extension(db_client): Extension<mongodb::Database>,
) -> Result<(StatusCode, Json<UserRegister>), UserError> {
    let user = form.0;

    // Check if phone already exists in DB
    if db_client
        .collection::<UserBase>("User")
        .find_one(doc! { "phone": &user.phone }, None)
        .await?
        .is_some()
    {
        return Err(UserError::AlreadyExists);
    } else {
        let token = generate_token();
        let user_base = UserBase {
            name: user.name.clone(),
            phone: user.phone.clone(),
            password: token.to_string(),
        };
        db_client
            .collection::<UserBase>("User")
            .insert_one(user_base, None)
            .await
            .map_err(|_| UserError::NotCreated)?;
    }

    Ok((StatusCode::CREATED, user.into()))
}

pub async fn post_login(
    form: Form<UserLogin>,
    Extension(db_client): Extension<mongodb::Database>,
) -> Result<(StatusCode, String), UserError> {
    let users = db_client.collection::<UserBase>("User");
    let login_data = form.0;

    if let Some(user) = users
        .find_one(doc! {"phone": login_data.phone}, None)
        .await?
    {
        if user.password == login_data.password {
            let jwt_header = Header::new(Algorithm::HS256);
            let jwt_claim = JwtClaim {
                exp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .checked_add(Duration::new(600, 0))
                    .unwrap()
                    .as_secs() as usize,
                jwt: user,
            };
            let jwt = encode(
                &jwt_header,
                &jwt_claim,
                &EncodingKey::from_secret(SECRET.as_ref()),
            );

            Ok((
                StatusCode::OK,
                jwt.map_or_else(|err| err.to_string(), |value| value),
            ))
        } else {
            Err(UserError::PasswordMismatch)
        }
    } else {
        Err(UserError::NotFound)
    }
}

pub async fn post_send_message(form: Form<Message>) -> Result<Json<Value>, JwtError> {
    let message = form.0;

    decode::<JwtClaim>(
        &message.jwt,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|err| match err.kind() {
        jsonwebtoken::errors::ErrorKind::InvalidSignature => JwtError::InvalidSignature,
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => JwtError::ExpiredSignature,
        _ => JwtError::InvalidToken,
    })?;

    Ok(Json(
        json!({"detail": "Message will be sent within 1 minute"}),
    ))
}

impl From<mongodb::error::Error> for UserError {
    fn from(error: mongodb::error::Error) -> Self {
        match *error.kind {
            mongodb::error::ErrorKind::Write(_) => UserError::NotCreated,
            _ => UserError::IO,
        }
    }
}

impl IntoResponse for JwtError {
    fn into_response(self) -> Response {
        let (status, err_message) = match self {
            JwtError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
            JwtError::InvalidSignature => (StatusCode::UNAUTHORIZED, "Invalid Signature"),
            JwtError::ExpiredSignature => (StatusCode::UNAUTHORIZED, "Signature Expired"),
        };

        let body = Json(json!({
            "error": err_message,
        }));

        (status, body).into_response()
    }
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let (status, err_message) = match self {
            UserError::AlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            UserError::NotFound => (StatusCode::NOT_FOUND, "User not found"),
            UserError::NotCreated => (StatusCode::UNPROCESSABLE_ENTITY, "User not created"),
            UserError::PasswordMismatch => (StatusCode::FORBIDDEN, "Password did not match"),
            UserError::IO => (StatusCode::REQUEST_TIMEOUT, "Request timed out"),
        };

        let body = Json(json!({
            "error": err_message,
        }));

        (status, body).into_response()
    }
}
