use dotenvy::dotenv;
use std::env;

#[derive(Clone)] 
pub struct Config {
    pub secret_key: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();  // Загружаем переменные из .env файла

        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

        Config { secret_key }
    }
}