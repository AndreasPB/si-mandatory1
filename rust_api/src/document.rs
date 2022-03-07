use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub phone: String,
    pub description: Option<String>,
}
