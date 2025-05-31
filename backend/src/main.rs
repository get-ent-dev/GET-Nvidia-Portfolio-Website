// backend/src/main.rs

use actix_web::{web, App, HttpServer};
use actix_session::{SessionMiddleware, config::PersistentSession, storage::CookieSessionStore};
use actix_web::cookie::Key;
use actix_cors::Cors;
use actix_files::Files;
use dotenvy::dotenv;
use std::env;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::mysql::MysqlConnection;
use listenfd::ListenFd;

// --- Module Declarations ---
mod handlers;
mod middleware;


// --- Database Connection Pool Type Alias ---
pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;

// --- Function to Establish Database Connection Pool ---
fn establish_connection_pool() -> DbPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create DB connection pool.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = establish_connection_pool();
    let secret_key = Key::generate();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_name("session_id".to_owned())
                    .cookie_secure(false) // Set to true in production with HTTPS
                    .session_lifecycle(
                        PersistentSession::default()
                            .session_ttl(actix_web::cookie::time::Duration::days(1))
                    )
                    .build()
            )
            // --- Public API Routes ---
            .service(
                web::scope("/api/v1")
                    .route("/projects", web::get().to(handlers::projects::get_all_projects))
                    .route("/projects/{id}", web::get().to(handlers::projects::get_project_by_id))
                    // CORRECTED: from get_about_content to get_about_page (based on common patterns and update_about_page)
                    .route("/about", web::get().to(handlers::about::get_about_page))
                    // CORRECTED: from submit_contact_form to submit_contact_message (assuming consistent naming)
                    .route("/contact", web::post().to(handlers::contact::submit_contact_message))
                    .route("/login", web::post().to(handlers::auth::login))
            )
            // --- Admin API Routes (Protected) ---
            .service(
                web::scope("/api/v1/admin")
                    .wrap(middleware::admin_auth::AdminAuth)
                    .route("/projects", web::post().to(handlers::projects::create_project))
                    .route("/projects/{id}", web::put().to(handlers::projects::update_project))
                    .route("/projects/{id}", web::delete().to(handlers::projects::delete_project))
                    // CORRECTED: from update_about_content to update_about_page (as suggested by compiler)
                    .route("/about", web::put().to(handlers::about::update_about_page))
                    // CORRECTED: from get_all_contact_forms to get_all_contact_messages (as suggested by compiler)
                    .route("/contact", web::get().to(handlers::contact::get_all_contact_messages))
                    // CORRECTED: from mark_contact_as_read to mark_contact_message_read (as suggested by compiler)
                    .route("/contact/{id}/read", web::put().to(handlers::contact::mark_contact_message_read))
                    // CORRECTED: from delete_contact_form to delete_contact_message (assuming consistent naming)
                    .route("/contact/{id}", web::delete().to(handlers::contact::delete_contact_message))
                    .route("/logout", web::post().to(handlers::auth::logout))
            )
            // --- Serve Frontend Static Files ---
            .service(Files::new("/", "../frontend/dist").index_file("index.html"))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0)? {
        server.listen(l)?
    } else {
        server.bind("0.0.0.0:8080")?
    };

    println!("🚀 Server running at http://0.0.0.0:8080");

    server.run().await
}
