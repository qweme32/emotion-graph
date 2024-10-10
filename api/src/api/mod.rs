mod user;

use crate::config::Config;
use actix_web::web;
use mongodb::Database;

pub fn admin_config(cfg: &mut web::ServiceConfig, _config: web::Data<Config>) {
    cfg.service(
        web::scope("/admin")
            .route("new-user", web::post().to(user::create_user)),
    );
}

pub fn user_config(
    cfg: &mut web::ServiceConfig,
    _config: web::Data<Config>,
    _db: web::Data<Database>,
) {
    cfg.service(
        web::scope("/user")
            .route("report", web::post().to(user::send_user_report))
            .route("authorize", web::post().to(user::authorize))
            .route("reports", web::post().to(user::get_reports))
            .route("me", web::get().to(user::get_me)),
    );
}
