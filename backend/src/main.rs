use axum::{
    routing::{delete, get, patch, post, put},
    Router,
};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

mod db;
mod handlers;
mod middleware;
mod models;

pub struct AppState {
    pub db: Arc<Mutex<rusqlite::Connection>>,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() {
    let conn = db::initialize_db().expect("Failed to initialize database");
    let state = Arc::new(AppState {
        db: Arc::new(Mutex::new(conn)),
        jwt_secret: std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "yatrasathi-dev-secret-2024".to_string()),
    });

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods(Any)
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ]);

    let app = Router::new()
        // Auth - new phone+password system
        .route("/api/auth/login", post(handlers::auth::login))
        .route("/api/auth/login/verify-otp", post(handlers::auth::login_verify_otp))
        .route("/api/auth/register", post(handlers::auth::register))
        .route("/api/auth/register/verify", post(handlers::auth::register_verify))
        .route("/api/auth/reset-password", post(handlers::auth::reset_password))
        .route("/api/auth/reset-password/verify", post(handlers::auth::reset_password_verify))
        // Auth - legacy OTP endpoints (backward compat)
        .route("/api/auth/otp/send", post(handlers::auth::send_otp))
        .route("/api/auth/otp/verify", post(handlers::auth::verify_otp))
        .route("/api/auth/admin/login", post(handlers::auth::admin_login))
        // Vehicles
        .route("/api/vehicles", get(handlers::vehicles::list_vehicles))
        .route("/api/vehicles", post(handlers::vehicles::create_vehicle))
        .route("/api/vehicles/search", post(handlers::vehicles::search_vehicles))
        .route("/api/vehicles/:id", get(handlers::vehicles::get_vehicle))
        .route("/api/vehicles/:id", put(handlers::vehicles::update_vehicle))
        .route("/api/vehicles/:id", delete(handlers::vehicles::delete_vehicle))
        // Bookings
        .route("/api/bookings", post(handlers::bookings::create_booking))
        .route("/api/bookings", get(handlers::bookings::list_bookings))
        .route("/api/bookings/:id", get(handlers::bookings::get_booking))
        .route("/api/bookings/:id/status", patch(handlers::bookings::update_booking_status))
        // Payments
        .route("/api/payments/esewa/verify", post(handlers::payments::verify_esewa))
        .route("/api/payments/khalti/verify", post(handlers::payments::verify_khalti))
        // Admin
        .route("/api/admin/stats", get(handlers::admin::get_stats))
        .route("/api/admin/users", get(handlers::admin::list_users))
        .route("/api/admin/vehicles", get(handlers::admin::list_all_vehicles))
        .route("/api/admin/bookings", get(handlers::admin::list_all_bookings))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Failed to bind to port 3001");

    println!("YatraSathi API running on http://localhost:3001");

    axum::serve(listener, app).await.unwrap();
}
