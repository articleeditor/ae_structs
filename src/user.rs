use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, CLone)]
pub struct User {
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: String,
    pub updated_at: String,
}