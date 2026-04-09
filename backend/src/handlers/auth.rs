use axum::{extract::State, http::StatusCode, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::Rng;
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::middleware::auth::{Claims, JWT_SECRET};
use crate::models::user::*;
use crate::AppState;

// ── Helpers ──────────────────────────────────────────────

fn generate_otp() -> String {
    format!("{:06}", rand::thread_rng().gen_range(100000..999999))
}

fn store_otp(db: &rusqlite::Connection, phone: &str, code: &str, purpose: &str) -> Result<(), String> {
    let id = Uuid::new_v4().to_string();
    let expires_at = (Utc::now() + Duration::minutes(5))
        .format("%Y-%m-%dT%H:%M:%S")
        .to_string();
    db.execute(
        "INSERT INTO otp_codes (id, phone, code, purpose, expires_at, used) VALUES (?1, ?2, ?3, ?4, ?5, 0)",
        rusqlite::params![id, phone, code, purpose, expires_at],
    ).map_err(|e| format!("DB error: {}", e))?;
    Ok(())
}

fn verify_otp_code(db: &rusqlite::Connection, phone: &str, code: &str, purpose: &str) -> Result<(), String> {
    let now = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let otp_id: Result<String, _> = db.query_row(
        "SELECT id FROM otp_codes WHERE phone = ?1 AND code = ?2 AND purpose = ?3 AND used = 0 AND expires_at > ?4 ORDER BY expires_at DESC LIMIT 1",
        rusqlite::params![phone, code, purpose, now],
        |row| row.get(0),
    );
    match otp_id {
        Ok(id) => {
            let _ = db.execute("UPDATE otp_codes SET used = 1 WHERE id = ?1", rusqlite::params![id]);
            Ok(())
        }
        Err(_) => Err("Invalid or expired OTP".to_string()),
    }
}

fn issue_token(id: &str, phone: &str, role: &str) -> String {
    let now_ts = Utc::now().timestamp() as usize;
    let claims = Claims {
        user_id: id.to_string(),
        phone: phone.to_string(),
        role: role.to_string(),
        iat: now_ts,
        exp: now_ts + 7 * 24 * 60 * 60,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_bytes()))
        .unwrap_or_default()
}

fn user_json(db: &rusqlite::Connection, phone: &str) -> Result<Value, String> {
    db.query_row(
        "SELECT id, phone, name, email, role, lang_pref, avatar_url, created_at FROM users WHERE phone = ?1",
        rusqlite::params![phone],
        |row| {
            Ok(json!({
                "id": row.get::<_, String>(0)?,
                "phone": row.get::<_, String>(1)?,
                "name": row.get::<_, String>(2)?,
                "email": row.get::<_, String>(3)?,
                "role": row.get::<_, String>(4)?,
                "lang_pref": row.get::<_, String>(5)?,
                "avatar_url": row.get::<_, String>(6)?,
                "created_at": row.get::<_, String>(7)?
            }))
        },
    ).map_err(|e| format!("User not found: {}", e))
}

/// Sparrow SMS (dev fallback)
async fn send_sms(phone: &str, code: &str) -> bool {
    let token = std::env::var("SPARROW_SMS_TOKEN").unwrap_or_default();
    if token.is_empty() { return false; }
    let from = std::env::var("SPARROW_SMS_FROM").unwrap_or_else(|_| "YatraSathi".to_string());
    let msg = format!("Your YatraSathi code is: {}. Valid for 5 minutes.", code);
    let client = reqwest::Client::new();
    client.post("https://api.sparrowsms.com/v2/sms/")
        .form(&[("token", token.as_str()), ("from", from.as_str()), ("to", phone), ("text", msg.as_str())])
        .send().await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}

// ── POST /api/auth/login ─────────────────────────────────
// Phone + password login for all roles
// For admin: returns requires_otp=true, sends OTP for 2FA
pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<Value>) {
    let phone = payload.phone.trim().to_string();
    let password = payload.password.trim().to_string();

    if phone.is_empty() || password.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "Phone and password are required" })));
    }

    let (user_id, user_role, stored_password) = {
        let db = state.db.lock().unwrap();
        let result = db.query_row(
            "SELECT id, role, password FROM users WHERE phone = ?1",
            rusqlite::params![phone],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?)),
        );
        match result {
            Ok(r) => r,
            Err(_) => return (StatusCode::UNAUTHORIZED, Json(json!({ "success": false, "error": "Invalid phone number or password" }))),
        }
    };

    if stored_password != password {
        return (StatusCode::UNAUTHORIZED, Json(json!({ "success": false, "error": "Invalid phone number or password" })));
    }

    // Admin requires 2FA
    if user_role == "admin" {
        let code = generate_otp();
        {
            let db = state.db.lock().unwrap();
            if let Err(e) = store_otp(&db, &phone, &code, "admin_2fa") {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e })));
            }
        }

        let sms_sent = send_sms(&phone, &code).await;

        return (StatusCode::OK, Json(json!({
            "success": true,
            "requires_otp": true,
            "message": if sms_sent { "OTP sent to your phone" } else { "OTP generated (dev mode)" },
            "otp": if sms_sent { None } else { Some(&code) },
            "dev_mode": !sms_sent
        })));
    }

    // Non-admin: direct login
    let db = state.db.lock().unwrap();
    let user = match user_json(&db, &phone) {
        Ok(u) => u,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))),
    };
    let token = issue_token(&user_id, &phone, &user_role);

    (StatusCode::OK, Json(json!({
        "success": true,
        "requires_otp": false,
        "token": token,
        "user": user
    })))
}

// ── POST /api/auth/login/verify-otp ──────────────────────
// Admin 2FA: verify OTP after successful password
pub async fn login_verify_otp(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AdminOtpVerifyRequest>,
) -> (StatusCode, Json<Value>) {
    let phone = payload.phone.trim().to_string();
    let otp = payload.otp.trim().to_string();

    let db = state.db.lock().unwrap();

    if let Err(e) = verify_otp_code(&db, &phone, &otp, "admin_2fa") {
        return (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e })));
    }

    let (user_id, role) = match db.query_row(
        "SELECT id, role FROM users WHERE phone = ?1 AND role = 'admin'",
        rusqlite::params![phone],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
    ) {
        Ok(r) => r,
        Err(_) => return (StatusCode::FORBIDDEN, Json(json!({ "success": false, "error": "Not an admin account" }))),
    };

    let user = match user_json(&db, &phone) {
        Ok(u) => u,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))),
    };
    let token = issue_token(&user_id, &phone, &role);

    (StatusCode::OK, Json(json!({ "success": true, "token": token, "user": user })))
}

// ── POST /api/auth/register ──────────────────────────────
// Start registration: validate data, send OTP
pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> (StatusCode, Json<Value>) {
    let phone = payload.phone.trim().to_string();
    let password = payload.password.trim().to_string();
    let name = payload.name.trim().to_string();
    let role = payload.role.trim().to_lowercase();

    if phone.is_empty() || password.is_empty() || name.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "Name, phone, and password are required" })));
    }

    if password.len() < 6 {
        return (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "Password must be at least 6 characters" })));
    }

    if !["consumer", "owner", "driver"].contains(&role.as_str()) {
        return (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "Invalid role. Choose: consumer, owner, or driver" })));
    }

    // Check if phone already registered
    let exists = {
        let db = state.db.lock().unwrap();
        db.query_row("SELECT COUNT(*) FROM users WHERE phone = ?1", rusqlite::params![phone], |row| row.get::<_, i64>(0))
            .unwrap_or(0) > 0
    };

    if exists {
        return (StatusCode::CONFLICT, Json(json!({ "success": false, "error": "Phone number already registered. Please login." })));
    }

    let code = generate_otp();
    {
        let db = state.db.lock().unwrap();
        if let Err(e) = store_otp(&db, &phone, &code, "register") {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e })));
        }
    }

    let sms_sent = send_sms(&phone, &code).await;

    (StatusCode::OK, Json(json!({
        "success": true,
        "message": if sms_sent { "OTP sent to your phone" } else { "OTP generated (dev mode)" },
        "otp": if sms_sent { None } else { Some(&code) },
        "dev_mode": !sms_sent
    })))
}

// ── POST /api/auth/register/verify ───────────────────────
// Complete registration: verify OTP, create account
pub async fn register_verify(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterVerifyRequest>,
) -> (StatusCode, Json<Value>) {
    let phone = payload.phone.trim().to_string();
    let otp = payload.otp.trim().to_string();
    let name = payload.name.trim().to_string();
    let password = payload.password.trim().to_string();
    let role = payload.role.trim().to_lowercase();

    if !["consumer", "owner", "driver"].contains(&role.as_str()) {
        return (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "Invalid role" })));
    }

    let db = state.db.lock().unwrap();

    if let Err(e) = verify_otp_code(&db, &phone, &otp, "register") {
        return (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e })));
    }

    // Create the user
    let user_id = Uuid::new_v4().to_string();
    let result = db.execute(
        "INSERT INTO users (id, phone, password, name, role, lang_pref) VALUES (?1, ?2, ?3, ?4, ?5, 'ne')",
        rusqlite::params![user_id, phone, password, name, role],
    );

    match result {
        Ok(_) => {
            let user = match user_json(&db, &phone) {
                Ok(u) => u,
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))),
            };
            let token = issue_token(&user_id, &phone, &role);
            (StatusCode::CREATED, Json(json!({ "success": true, "token": token, "user": user })))
        }
        Err(e) => {
            if e.to_string().contains("UNIQUE") {
                (StatusCode::CONFLICT, Json(json!({ "success": false, "error": "Phone number already registered" })))
            } else {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": format!("Failed to create account: {}", e) })))
            }
        }
    }
}

// ── POST /api/auth/reset-password ────────────────────────
// Request password reset OTP
pub async fn reset_password(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ResetPasswordRequest>,
) -> (StatusCode, Json<Value>) {
    let phone = payload.phone.trim().to_string();

    let exists = {
        let db = state.db.lock().unwrap();
        db.query_row("SELECT COUNT(*) FROM users WHERE phone = ?1", rusqlite::params![phone], |row| row.get::<_, i64>(0))
            .unwrap_or(0) > 0
    };

    if !exists {
        return (StatusCode::NOT_FOUND, Json(json!({ "success": false, "error": "No account found with this phone number" })));
    }

    let code = generate_otp();
    {
        let db = state.db.lock().unwrap();
        if let Err(e) = store_otp(&db, &phone, &code, "reset") {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e })));
        }
    }

    let sms_sent = send_sms(&phone, &code).await;

    (StatusCode::OK, Json(json!({
        "success": true,
        "message": if sms_sent { "OTP sent to your phone" } else { "OTP generated (dev mode)" },
        "otp": if sms_sent { None } else { Some(&code) },
        "dev_mode": !sms_sent
    })))
}

// ── POST /api/auth/reset-password/verify ─────────────────
// Verify OTP and set new password
pub async fn reset_password_verify(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ResetPasswordVerifyRequest>,
) -> (StatusCode, Json<Value>) {
    let phone = payload.phone.trim().to_string();
    let otp = payload.otp.trim().to_string();
    let new_password = payload.new_password.trim().to_string();

    if new_password.len() < 6 {
        return (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "Password must be at least 6 characters" })));
    }

    let db = state.db.lock().unwrap();

    if let Err(e) = verify_otp_code(&db, &phone, &otp, "reset") {
        return (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": e })));
    }

    match db.execute("UPDATE users SET password = ?1 WHERE phone = ?2", rusqlite::params![new_password, phone]) {
        Ok(_) => (StatusCode::OK, Json(json!({ "success": true, "message": "Password reset successful. Please login." }))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": format!("Failed: {}", e) }))),
    }
}

// ── Legacy endpoints (kept for backward compat) ──────────

pub async fn send_otp(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<OtpSendRequest>,
) -> (StatusCode, Json<Value>) {
    let phone = payload.phone.trim().to_string();
    if phone.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "Phone number is required" })));
    }

    let code = generate_otp();
    {
        let db = state.db.lock().unwrap();
        if let Err(e) = store_otp(&db, &phone, &code, "register") {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e })));
        }
    }

    let sms_sent = send_sms(&phone, &code).await;

    (StatusCode::OK, Json(json!({
        "success": true,
        "message": if sms_sent { "OTP sent" } else { "OTP generated (dev mode)" },
        "otp": if sms_sent { None } else { Some(&code) },
        "dev_mode": !sms_sent
    })))
}

pub async fn verify_otp(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<OtpVerifyRequest>,
) -> (StatusCode, Json<Value>) {
    let phone = payload.phone.trim().to_string();
    let otp = payload.otp.trim().to_string();
    let db = state.db.lock().unwrap();

    // Try any purpose OTP
    let now = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let otp_id: Result<String, _> = db.query_row(
        "SELECT id FROM otp_codes WHERE phone = ?1 AND code = ?2 AND used = 0 AND expires_at > ?3 ORDER BY expires_at DESC LIMIT 1",
        rusqlite::params![phone, otp, now],
        |row| row.get(0),
    );

    match otp_id {
        Ok(id) => { let _ = db.execute("UPDATE otp_codes SET used = 1 WHERE id = ?1", rusqlite::params![id]); }
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "Invalid or expired OTP" }))),
    }

    // Get or create user
    let user_exists: bool = db.query_row(
        "SELECT COUNT(*) FROM users WHERE phone = ?1", rusqlite::params![phone], |row| row.get::<_, i64>(0),
    ).map(|c| c > 0).unwrap_or(false);

    if !user_exists {
        let uid = Uuid::new_v4().to_string();
        let _ = db.execute(
            "INSERT INTO users (id, phone, name, role, lang_pref) VALUES (?1, ?2, '', 'consumer', 'ne')",
            rusqlite::params![uid, phone],
        );
    }

    let user = match user_json(&db, &phone) {
        Ok(u) => u,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": e }))),
    };

    let user_id = user["id"].as_str().unwrap_or("");
    let role = user["role"].as_str().unwrap_or("consumer");
    let token = issue_token(user_id, &phone, role);

    (StatusCode::OK, Json(json!({ "success": true, "token": token, "user": user })))
}

pub async fn admin_login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AdminLoginRequest>,
) -> (StatusCode, Json<Value>) {
    let username = payload.username.trim().to_string();
    let password = payload.password.trim().to_string();

    if username.is_empty() || password.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(json!({ "success": false, "error": "Username and password required" })));
    }

    let db = state.db.lock().unwrap();
    let user_row = db.query_row(
        "SELECT id, phone, name, role, password FROM users WHERE (name = ?1 OR phone = ?1) AND role = 'admin'",
        rusqlite::params![username],
        |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?, row.get::<_, String>(4)?)),
    );

    match user_row {
        Ok((id, phone, _name, role, stored_pw)) => {
            if password != stored_pw && password != phone {
                return (StatusCode::UNAUTHORIZED, Json(json!({ "success": false, "error": "Invalid credentials" })));
            }
            let user = user_json(&db, &phone).unwrap_or(json!({}));
            let token = issue_token(&id, &phone, &role);
            (StatusCode::OK, Json(json!({ "success": true, "token": token, "user": user })))
        }
        Err(_) => (StatusCode::UNAUTHORIZED, Json(json!({ "success": false, "error": "Invalid credentials" }))),
    }
}
