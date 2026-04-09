use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::middleware::auth::AuthUser;
use crate::models::payment::{EsewaVerifyRequest, KhaltiVerifyRequest};
use crate::AppState;

/// POST /api/payments/esewa/verify
/// Verify eSewa payment transaction after redirect callback
pub async fn verify_esewa(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
    Json(payload): Json<EsewaVerifyRequest>,
) -> (StatusCode, Json<Value>) {
    let db = state.db.lock().unwrap();

    // Validate the booking exists and belongs to the user
    let booking_info = db.query_row(
        "SELECT renter_id, total_amount, status, payment_method FROM bookings WHERE id = ?1",
        rusqlite::params![payload.booking_id],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, f64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        },
    );

    let (renter_id, total_amount, status, payment_method) = match booking_info {
        Ok(info) => info,
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "success": false, "error": "Booking not found" })),
            );
        }
    };

    if renter_id != claims.user_id && claims.role != "admin" {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "success": false, "error": "Access denied" })),
        );
    }

    if status != "pending" {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "Booking is not in pending status" })),
        );
    }

    if payment_method != "esewa" {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "Booking payment method is not eSewa" })),
        );
    }

    // eSewa verification logic
    // In production: call eSewa's transaction verification API
    // https://developer.esewa.com.np/pages/Epay#transrec
    // POST to https://uat.esewa.com.np/epay/transrec with:
    //   amt, scd (merchant code), pid (product id), rid (reference id)
    //
    // For development: we validate the transaction data structure and simulate success

    // Validate amount matches
    let esewa_amount = payload.amount.unwrap_or(0.0);
    if (esewa_amount - total_amount).abs() > 1.0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "Payment amount mismatch",
                "expected": total_amount,
                "received": esewa_amount
            })),
        );
    }

    // Validate required eSewa fields
    if payload.reference_id.is_none() || payload.product_id.is_none() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "Missing eSewa reference_id or product_id"
            })),
        );
    }

    // In dev mode: auto-verify. In production: call eSewa API to verify transaction
    // Update booking status to confirmed
    let result = db.execute(
        "UPDATE bookings SET status = 'confirmed' WHERE id = ?1",
        rusqlite::params![payload.booking_id],
    );

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "eSewa payment verified successfully",
                "booking_id": payload.booking_id,
                "amount": total_amount,
                "payment_method": "esewa",
                "status": "confirmed",
                "transaction": {
                    "reference_id": payload.reference_id,
                    "product_id": payload.product_id,
                    "gateway": "esewa"
                }
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "error": format!("Failed to update booking: {}", e) })),
        ),
    }
}

/// POST /api/payments/khalti/verify
/// Verify Khalti payment after client-side payment completion
pub async fn verify_khalti(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
    Json(payload): Json<KhaltiVerifyRequest>,
) -> (StatusCode, Json<Value>) {
    let db = state.db.lock().unwrap();

    // Validate the booking exists and belongs to the user
    let booking_info = db.query_row(
        "SELECT renter_id, total_amount, status, payment_method FROM bookings WHERE id = ?1",
        rusqlite::params![payload.booking_id],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, f64>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        },
    );

    let (renter_id, total_amount, status, payment_method) = match booking_info {
        Ok(info) => info,
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "success": false, "error": "Booking not found" })),
            );
        }
    };

    if renter_id != claims.user_id && claims.role != "admin" {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "success": false, "error": "Access denied" })),
        );
    }

    if status != "pending" {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "Booking is not in pending status" })),
        );
    }

    if payment_method != "khalti" {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "Booking payment method is not Khalti" })),
        );
    }

    // Khalti verification logic
    // In production: call Khalti's lookup API
    // POST to https://khalti.com/api/v2/payment/verify/ with:
    //   token, amount (in paisa)
    // Headers: Authorization: Key <KHALTI_SECRET_KEY>
    //
    // For development: validate structure and simulate success

    if payload.token.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "Missing Khalti payment token" })),
        );
    }

    // Validate amount (Khalti uses paisa, 1 NPR = 100 paisa)
    let khalti_amount_npr = payload.amount.unwrap_or(0.0) / 100.0;
    if (khalti_amount_npr - total_amount).abs() > 1.0 && payload.amount.unwrap_or(0.0) != total_amount {
        // Accept both NPR and paisa amounts for flexibility
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "Payment amount mismatch",
                "expected_npr": total_amount
            })),
        );
    }

    // Update booking status to confirmed
    let result = db.execute(
        "UPDATE bookings SET status = 'confirmed' WHERE id = ?1",
        rusqlite::params![payload.booking_id],
    );

    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "message": "Khalti payment verified successfully",
                "booking_id": payload.booking_id,
                "amount": total_amount,
                "payment_method": "khalti",
                "status": "confirmed",
                "transaction": {
                    "token": payload.token,
                    "gateway": "khalti"
                }
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "success": false, "error": format!("Failed to update booking: {}", e) })),
        ),
    }
}
