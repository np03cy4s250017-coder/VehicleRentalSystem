use axum::{extract::State, http::StatusCode, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use rand::Rng;
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::middleware::auth::{Claims, JWT_SECRET};
use crate::models::user::{OtpSendRequest, OtpVerifyRequest};
use crate::AppState;

pub async fn send_otp(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<OtpSendRequest>,
) -> (StatusCode, Json<Value>) {
    let phone = payload.phone.trim().to_string();
    if phone.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "Phone number is required" })),
        );
    }

    let code: String = format!("{:06}", rand::thread_rng().gen_range(100000..999999));
    let otp_id = Uuid::new_v4().to_string();
    let expires_at = (Utc::now() + Duration::minutes(5))
        .format("%Y-%m-%dT%H:%M:%S")
        .to_string();

    let db = state.db.lock().unwrap();
    let result = db.execute(
        "INSERT INTO otp_codes (id, phone, code, expires_at, used) VALUES (?1, ?2, ?3, ?4, 0)",
        rusqlite::params![otp_id, phone, code, expires_at],
    );

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "OTP sent successfully",
                "otp": code
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "error": format!("Failed to send OTP: {}", e) })),
        ),
    }
}

pub async fn verify_otp(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<OtpVerifyRequest>,
) -> (StatusCode, Json<Value>) {
    let phone = payload.phone.trim().to_string();
    let otp = payload.otp.trim().to_string();

    let db = state.db.lock().unwrap();

    // Find valid OTP
    let now = Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let otp_result: Result<(String,), _> = db.query_row(
        "SELECT id FROM otp_codes WHERE phone = ?1 AND code = ?2 AND used = 0 AND expires_at > ?3 ORDER BY expires_at DESC LIMIT 1",
        rusqlite::params![phone, otp, now],
        |row| Ok((row.get(0)?,)),
    );

    let otp_id = match otp_result {
        Ok((id,)) => id,
        Err(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "error": "Invalid or expired OTP" })),
            );
        }
    };

    // Mark OTP as used
    let _ = db.execute(
        "UPDATE otp_codes SET used = 1 WHERE id = ?1",
        rusqlite::params![otp_id],
    );

    // Get or create user
    let user_exists: bool = db
        .query_row(
            "SELECT COUNT(*) FROM users WHERE phone = ?1",
            rusqlite::params![phone],
            |row| row.get::<_, i64>(0),
        )
        .map(|c| c > 0)
        .unwrap_or(false);

    if !user_exists {
        let user_id = Uuid::new_v4().to_string();
        let _ = db.execute(
            "INSERT INTO users (id, phone, name, email, role, lang_pref, avatar_url) VALUES (?1, ?2, '', '', 'renter', 'ne', '')",
            rusqlite::params![user_id, phone],
        );
    }

    // Fetch user
    let user_row = db.query_row(
        "SELECT id, phone, name, email, role, lang_pref, avatar_url, created_at FROM users WHERE phone = ?1",
        rusqlite::params![phone],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, String>(6)?,
                row.get::<_, String>(7)?,
            ))
        },
    );

    match user_row {
        Ok((id, phone, name, email, role, lang_pref, avatar_url, created_at)) => {
            let now_ts = Utc::now().timestamp() as usize;
            let claims = Claims {
                user_id: id.clone(),
                phone: phone.clone(),
                role: role.clone(),
                iat: now_ts,
                exp: now_ts + 7 * 24 * 60 * 60, // 7 days
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
            )
            .unwrap_or_default();

            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "token": token,
                    "user": {
                        "id": id,
                        "phone": phone,
                        "name": name,
                        "email": email,
                        "role": role,
                        "lang_pref": lang_pref,
                        "avatar_url": avatar_url,
                        "created_at": created_at
                    }
                })),
            )
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "error": format!("Database error: {}", e) })),
        ),
    }
}
