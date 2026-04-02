use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::middleware::auth::AuthUser;
use crate::models::booking::{CreateBookingRequest, UpdateBookingStatusRequest};
use crate::AppState;

fn row_to_booking(row: &rusqlite::Row) -> rusqlite::Result<Value> {
    Ok(json!({
        "id": row.get::<_, String>(0)?,
        "renter_id": row.get::<_, String>(1)?,
        "vehicle_id": row.get::<_, String>(2)?,
        "driver_id": row.get::<_, Option<String>>(3)?,
        "start_time": row.get::<_, String>(4)?,
        "end_time": row.get::<_, String>(5)?,
        "total_amount": row.get::<_, f64>(6)?,
        "payment_method": row.get::<_, String>(7)?,
        "status": row.get::<_, String>(8)?,
        "carbon_saved_kg": row.get::<_, f64>(9)?,
        "created_at": row.get::<_, String>(10)?,
    }))
}

pub async fn create_booking(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
    Json(payload): Json<CreateBookingRequest>,
) -> (StatusCode, Json<Value>) {
    let db = state.db.lock().unwrap();

    // Get vehicle rates and EV status
    let vehicle_info = db.query_row(
        "SELECT hourly_rate, daily_rate, is_ev, available FROM vehicles WHERE id = ?1",
        rusqlite::params![payload.vehicle_id],
        |row| {
            Ok((
                row.get::<_, Option<f64>>(0)?,
                row.get::<_, Option<f64>>(1)?,
                row.get::<_, i32>(2)?,
                row.get::<_, i32>(3)?,
            ))
        },
    );

    let (hourly_rate, daily_rate, is_ev, available) = match vehicle_info {
        Ok(info) => info,
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Vehicle not found" })),
            );
        }
    };

    if available == 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Vehicle is not available" })),
        );
    }

    // Parse times and calculate amount
    let start = NaiveDateTime::parse_from_str(&payload.start_time, "%Y-%m-%dT%H:%M:%S");
    let end = NaiveDateTime::parse_from_str(&payload.end_time, "%Y-%m-%dT%H:%M:%S");

    let (start_dt, end_dt) = match (start, end) {
        (Ok(s), Ok(e)) => (s, e),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "Invalid date format. Use YYYY-MM-DDTHH:MM:SS" })),
            );
        }
    };

    let duration = end_dt.signed_duration_since(start_dt);
    let hours = duration.num_hours() as f64;

    if hours <= 0.0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "End time must be after start time" })),
        );
    }

    let total_amount = if hours >= 24.0 {
        let days = (hours / 24.0).ceil();
        daily_rate.unwrap_or(0.0) * days
    } else {
        let h = hours.ceil();
        hourly_rate.unwrap_or(0.0) * h
    };

    // Estimate carbon saved (approx 0.12 kg CO2 per km for ICE, assume ~70km/day for EV)
    let carbon_saved_kg = if is_ev == 1 {
        let days = (hours / 24.0).max(1.0);
        days * 0.12 * 70.0
    } else {
        0.0
    };

    let booking_id = Uuid::new_v4().to_string();
    let payment_method = payload.payment_method.unwrap_or_else(|| "esewa".to_string());

    let result = db.execute(
        "INSERT INTO bookings (id, renter_id, vehicle_id, start_time, end_time, total_amount, payment_method, status, carbon_saved_kg) VALUES (?1,?2,?3,?4,?5,?6,?7,'pending',?8)",
        rusqlite::params![
            booking_id,
            claims.user_id,
            payload.vehicle_id,
            payload.start_time,
            payload.end_time,
            total_amount,
            payment_method,
            carbon_saved_kg,
        ],
    );

    match result {
        Ok(_) => {
            let booking = db.query_row(
                "SELECT id, renter_id, vehicle_id, driver_id, start_time, end_time, total_amount, payment_method, status, carbon_saved_kg, created_at FROM bookings WHERE id = ?1",
                rusqlite::params![booking_id],
                |row| row_to_booking(row),
            );
            match booking {
                Ok(b) => (StatusCode::CREATED, Json(json!({ "booking": b }))),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("Created but failed to fetch: {}", e) })),
                ),
            }
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Failed to create booking: {}", e) })),
        ),
    }
}

pub async fn list_bookings(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
) -> (StatusCode, Json<Value>) {
    let db = state.db.lock().unwrap();

    let mut stmt = db.prepare(
        "SELECT b.id, b.renter_id, b.vehicle_id, b.driver_id, b.start_time, b.end_time, b.total_amount, b.payment_method, b.status, b.carbon_saved_kg, b.created_at, v.make, v.model, v.type, v.image_url, v.plate_no FROM bookings b LEFT JOIN vehicles v ON b.vehicle_id = v.id WHERE b.renter_id = ?1 ORDER BY b.created_at DESC"
    ).unwrap();

    let bookings: Vec<Value> = stmt
        .query_map(rusqlite::params![claims.user_id], |row| {
            Ok(json!({
                "id": row.get::<_, String>(0)?,
                "renter_id": row.get::<_, String>(1)?,
                "vehicle_id": row.get::<_, String>(2)?,
                "driver_id": row.get::<_, Option<String>>(3)?,
                "start_time": row.get::<_, String>(4)?,
                "end_time": row.get::<_, String>(5)?,
                "total_amount": row.get::<_, f64>(6)?,
                "payment_method": row.get::<_, String>(7)?,
                "status": row.get::<_, String>(8)?,
                "carbon_saved_kg": row.get::<_, f64>(9)?,
                "created_at": row.get::<_, String>(10)?,
                "vehicle": {
                    "make": row.get::<_, Option<String>>(11)?,
                    "model": row.get::<_, Option<String>>(12)?,
                    "type": row.get::<_, Option<String>>(13)?,
                    "image_url": row.get::<_, Option<String>>(14)?,
                    "plate_no": row.get::<_, Option<String>>(15)?
                }
            }))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    (StatusCode::OK, Json(json!({ "bookings": bookings })))
}

pub async fn get_booking(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    let db = state.db.lock().unwrap();

    let result = db.query_row(
        "SELECT b.id, b.renter_id, b.vehicle_id, b.driver_id, b.start_time, b.end_time, b.total_amount, b.payment_method, b.status, b.carbon_saved_kg, b.created_at, v.make, v.model, v.type, v.image_url, v.plate_no, v.location_name FROM bookings b LEFT JOIN vehicles v ON b.vehicle_id = v.id WHERE b.id = ?1",
        rusqlite::params![id],
        |row| {
            Ok(json!({
                "id": row.get::<_, String>(0)?,
                "renter_id": row.get::<_, String>(1)?,
                "vehicle_id": row.get::<_, String>(2)?,
                "driver_id": row.get::<_, Option<String>>(3)?,
                "start_time": row.get::<_, String>(4)?,
                "end_time": row.get::<_, String>(5)?,
                "total_amount": row.get::<_, f64>(6)?,
                "payment_method": row.get::<_, String>(7)?,
                "status": row.get::<_, String>(8)?,
                "carbon_saved_kg": row.get::<_, f64>(9)?,
                "created_at": row.get::<_, String>(10)?,
                "vehicle": {
                    "make": row.get::<_, Option<String>>(11)?,
                    "model": row.get::<_, Option<String>>(12)?,
                    "type": row.get::<_, Option<String>>(13)?,
                    "image_url": row.get::<_, Option<String>>(14)?,
                    "plate_no": row.get::<_, Option<String>>(15)?,
                    "location_name": row.get::<_, Option<String>>(16)?
                }
            }))
        },
    );

    match result {
        Ok(booking) => {
            // Verify the user owns this booking or is admin
            if let Some(renter_id) = booking["renter_id"].as_str() {
                if renter_id != claims.user_id && claims.role != "admin" {
                    return (
                        StatusCode::FORBIDDEN,
                        Json(json!({ "error": "Access denied" })),
                    );
                }
            }
            (StatusCode::OK, Json(json!({ "booking": booking })))
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Booking not found" })),
        ),
    }
}

pub async fn update_booking_status(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
    Path(id): Path<String>,
    Json(payload): Json<UpdateBookingStatusRequest>,
) -> (StatusCode, Json<Value>) {
    let valid_statuses = ["pending", "confirmed", "active", "completed", "cancelled"];
    if !valid_statuses.contains(&payload.status.as_str()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Invalid status. Must be one of: {:?}", valid_statuses) })),
        );
    }

    let db = state.db.lock().unwrap();

    // Verify booking exists and user has access
    let renter_id: Result<String, _> = db.query_row(
        "SELECT renter_id FROM bookings WHERE id = ?1",
        rusqlite::params![id],
        |row| row.get(0),
    );

    match renter_id {
        Ok(rid) => {
            if rid != claims.user_id && claims.role != "admin" {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({ "error": "Access denied" })),
                );
            }
        }
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Booking not found" })),
            );
        }
    }

    match db.execute(
        "UPDATE bookings SET status = ?1 WHERE id = ?2",
        rusqlite::params![payload.status, id],
    ) {
        Ok(_) => {
            let booking = db.query_row(
                "SELECT id, renter_id, vehicle_id, driver_id, start_time, end_time, total_amount, payment_method, status, carbon_saved_kg, created_at FROM bookings WHERE id = ?1",
                rusqlite::params![id],
                |row| row_to_booking(row),
            );
            match booking {
                Ok(b) => (StatusCode::OK, Json(json!({ "booking": b }))),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("Updated but failed to fetch: {}", e) })),
                ),
            }
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Failed to update status: {}", e) })),
        ),
    }
}
