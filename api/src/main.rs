mod models;
mod config;
mod api;
mod utils;
mod dto;

use config::Config;
use mongodb::Client;
use actix_web::{web, App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = Client::with_uri_str("mongodb://mongo:27017")
        .await
        .expect("Failed to initialize MongoDB client");

    let config = Config::from_env();

    let db = client.database("emotions");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(config.clone()))
            .configure(|cfg| 
                api::admin_config(cfg, web::Data::new(
                    config.clone()
                ).clone())
            )
            .configure(|cfg| 
                api::user_config(
                    cfg, 
                    web::Data::new(
                        config.clone()
                    ).clone(),
                    web::Data::new(
                        db.clone()
                    ).clone()
                )
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}