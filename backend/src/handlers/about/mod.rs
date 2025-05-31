// backend/src/handlers/about/mod.rs
use actix_web::{web, HttpResponse, Responder};
use crate::DbPool;

// Placeholder for get_about_page
pub async fn get_about_page(_pool: web::Data<DbPool>) -> impl Responder {
    HttpResponse::Ok().body("About page content (placeholder)")
}

// Placeholder for update_about_page
pub async fn update_about_page(_content: web::Json<serde_json::Value>, _pool: web::Data<DbPool>) -> impl Responder {
    HttpResponse::Ok().body("About page updated (placeholder)")
}
