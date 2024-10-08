use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

// Структура для данных, которые будут храниться в JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct UserJwt {
    pub sub: String, // Поле с идентификатором пользователя
    pub exp: usize,  // Время истечения токена (в секундах с 1970 года)
    pub is_valid: bool,
}

impl UserJwt {
    pub fn new(subject: &str) -> Self {
        Self {
            sub: subject.to_owned(),
            exp: 0,
            is_valid: true,
        }
    }

    pub fn sign(&mut self, secret_key: &str, ttl: usize) -> String {
        self.exp = chrono::Utc::now().timestamp() as usize + ttl;

        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret_key.as_ref()),
        )
        .unwrap()
    }

    pub fn from(token: &str, secret_key: &str) -> Self {
        let validation = Validation::new(Algorithm::HS256);
        match decode::<Self>(
            token,
            &DecodingKey::from_secret(secret_key.as_ref()),
            &validation,
        ) {
            Ok(data) => Self {
                sub: data.claims.sub,
                exp: data.claims.exp,
                is_valid: true,
            },
            Err(_) => {
                Self {
                    sub: String::from("invalid_token"),
                    exp: 0,
                    is_valid: false,
                }
            }
        }
    }
}
