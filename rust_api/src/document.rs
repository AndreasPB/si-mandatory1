use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserBase {
    pub name: String,
    pub phone: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRegister {
    pub name: String,
    pub phone: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLogin {
    phone: String,
    token: String,
}
