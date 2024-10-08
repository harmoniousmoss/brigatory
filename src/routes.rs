use actix_web::{web, HttpResponse};

use crate::handlers::{
    add_blacklist_ip,
    add_blacklist_url,
    check_rate_limit, // Import the check_rate_limit handler
    delete_blacklist_ip_by_id,
    delete_blacklist_url_by_id,
    edit_blacklist_ip_by_id,
    edit_blacklist_url_by_id,
    get_all_blacklist_ip,
    get_all_blacklist_url,
    get_blacklist_ip_by_id,
    get_blacklist_url_by_id,
    is_blacklist_ip,
    is_blacklist_url,
    signin,
    signup,
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Rate limiting endpoint
        .service(web::resource("/check-rate-limit").route(web::post().to(check_rate_limit)))
        // Blacklist IP endpoints
        .service(
            web::resource("/blacklist-ip")
                .route(web::post().to(add_blacklist_ip))
                .route(web::get().to(get_all_blacklist_ip)),
        )
        .service(
            web::resource("/blacklist-ip/{id}")
                .route(web::get().to(get_blacklist_ip_by_id))
                .route(web::delete().to(delete_blacklist_ip_by_id))
                .route(web::put().to(edit_blacklist_ip_by_id)),
        )
        .service(web::resource("/check-blacklist-ip").route(web::post().to(is_blacklist_ip)))
        // Blacklist URL endpoints
        .service(
            web::resource("/blacklist-url")
                .route(web::post().to(add_blacklist_url))
                .route(web::get().to(get_all_blacklist_url)),
        )
        .service(
            web::resource("/blacklist-url/{id}")
                .route(web::get().to(get_blacklist_url_by_id))
                .route(web::delete().to(delete_blacklist_url_by_id))
                .route(web::put().to(edit_blacklist_url_by_id)),
        )
        .service(web::resource("/check-blacklist-url").route(web::post().to(is_blacklist_url)))
        // User endpoints
        .service(web::resource("/signup").route(web::post().to(signup)))
        .service(web::resource("/signin").route(web::post().to(signin)));
}

pub fn configure_greet(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(|| async { HttpResponse::Ok().body("Brigatory Here") })),
    );
}
