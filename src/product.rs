use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Clone, Validate)]
pub struct User {
    #[validate(email)]
    pub email: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: String,
    pub updated_at: String,
}
