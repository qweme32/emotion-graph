use actix_web::http::header;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use chrono::{Duration, Utc};
use futures_util::TryStreamExt;
use mongodb::bson::{doc, DateTime};
use mongodb::{Collection, Database};

use crate::config::Config;
use crate::dto;
use crate::models::report::Report;
use crate::models::user::User;
use crate::utils::jwt::UserJwt;
use serde_json::json;

pub async fn create_user(
    db: web::Data<Database>,
    config: web::Data<Config>,
    dto: actix_web_validator::Json<dto::user::AuthDto>,
    query: actix_web_validator::Query<dto::admin::SecretDto>,
) -> impl Responder {
    if query.secret != config.secret_key {
        return HttpResponse::Unauthorized().json(json!({ "ok": false }));
    }

    let collection: Collection<User> = db.collection("users");

    let new_user = User {
        id: None,
        username: dto.username.clone(),
        password: dto.password.clone(),
    };

    let filter = doc! { "username": new_user.username.clone() };

    match collection.find_one(filter).await.unwrap() {
        Some(_) => HttpResponse::Conflict().json(json!({ "ok": false })),
        None => {
            collection
                .insert_one(new_user)
                .await
                .expect("Failed to insert item.");

            HttpResponse::Ok().json(json!({ "ok": true }))
        }
    }
}

pub async fn authorize(
    db: web::Data<Database>,
    config: web::Data<Config>,
    report: web::Json<dto::user::AuthDto>,
) -> impl Responder {
    let users: Collection<User> = db.collection("users");

    let user: Option<User> = users
        .find_one(doc! {
            "username": report.username.clone(),
            "password": report.password.clone()
        })
        .await
        .unwrap();

    match user {
        Some(user) => {
            let token = UserJwt::new(&user.username).sign(&config.secret_key, 60 * 60 * 24 * 7);

            HttpResponse::Ok().json(json!({
                "ok": true,
                "token": token,
                "ttl": 60 * 60 * 24 * 7
                }
            ))
        }
        None => HttpResponse::Unauthorized().json(json!({
            "ok": false,
            "token": null,
            "ttl": 0
        })),
    }
}

pub async fn get_reports(
    db: web::Data<Database>,
    data: web::Json<dto::user::ReportsDto>,
) -> impl Responder {
    let users: Collection<User> = db.collection("users");
    let reports: Collection<Report> = db.collection("reports");

    let user = users
        .find_one(doc! { "username": data.username.clone() })
        .await
        .unwrap();

    if user.is_none() {
        return HttpResponse::NotFound().json(json!({ "ok": false, "reports": [] }));
    }

    let items: Vec<Report> = reports
        .find(doc! {
            "date": {
                "$gte": DateTime::from_millis(
                    (
                        Utc::now() - Duration::days(data.period as i64)
                    ).timestamp_millis()
                )
            },
            "author": user.unwrap().id
        })
        .await
        .unwrap()
        .try_collect()
        .await
        .unwrap();

    HttpResponse::Ok().json(json!({ "ok": true, "reports": items }))
}

pub async fn send_user_report(
    db: web::Data<Database>,
    config: web::Data<Config>, // Получаем доступ к конфигурации
    report: web::Json<dto::user::ReportDto>,
    req: HttpRequest,
) -> impl Responder {
    let users: Collection<User> = db.collection("users");
    let reports: Collection<Report> = db.collection("reports");

    let mut user: Option<User> = None;

    if let Some(auth_header) = req.headers().get(header::AUTHORIZATION) {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let jwt = UserJwt::from(token, &config.secret_key);

                if jwt.is_valid == true {
                    let filter = doc! { "username": jwt.sub };

                    user = users.find_one(filter).await.unwrap();
                }
            }
        }
    }

    match user {
        Some(user) => {
            let new_report = Report {
                id: None,
                author: user.id.unwrap(),
                date: DateTime::now(),
                rate: report.rate,
            };

            reports.insert_one(new_report).await.unwrap();

            HttpResponse::Ok().json(json!({ "ok": true }))
        }
        None => HttpResponse::Unauthorized().json(json!({ "ok": false })),
    }
}
