use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBase {
    pub name: String,
    pub phone: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRegister {
    pub name: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLogin {
    pub phone: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message: String,
    pub to_phone: String,
    pub jwt: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtClaim {
    pub exp: usize,
    pub jwt: UserBase,
}
