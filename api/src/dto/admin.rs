use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SecretDto {
    #[validate(length(min = 1, max = 32, message = "Secret should have 1-32 characters"))]
    pub secret: String,
}