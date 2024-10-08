use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AuthDto {
    #[validate(length(min = 3, max = 12, message = "Username should have 3-12 characters"))]
    pub username: String,

    #[validate(length(min = 8, max = 32, message = "Password should have 8-32 characters"))]
    pub password: String
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ReportDto {
    #[validate(range(min = -3, max = 3, message = "Rate must be between -3 and 3"))]
    pub rate: i32
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ReportsDto {
    #[validate(range(min = 1, max = 30, message = "Period must be between 1 and 30"))]
    pub period: i32,

    #[validate(length(min = 3, max = 12, message = "Username should have 3-12 characters"))]
    pub username: String,
}