use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::middleware::auth::AuthUser;
use crate::AppState;

pub async fn get_stats(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
) -> (StatusCode, Json<Value>) {
    if claims.role != "admin" {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Admin access required" })),
        );
    }

    let db = state.db.lock().unwrap();

    let total_users: i64 = db
        .query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))
        .unwrap_or(0);

    let total_vehicles: i64 = db
        .query_row("SELECT COUNT(*) FROM vehicles", [], |row| row.get(0))
        .unwrap_or(0);

    let total_bookings: i64 = db
        .query_row("SELECT COUNT(*) FROM bookings", [], |row| row.get(0))
        .unwrap_or(0);

    let total_revenue: f64 = db
        .query_row(
            "SELECT COALESCE(SUM(total_amount), 0) FROM bookings WHERE status IN ('confirmed','active','completed')",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0.0);

    // Recent bookings
    let mut stmt = db.prepare(
        "SELECT b.id, b.renter_id, b.vehicle_id, b.driver_id, b.start_time, b.end_time, b.total_amount, b.payment_method, b.status, b.carbon_saved_kg, b.created_at, v.make, v.model, u.name as renter_name FROM bookings b LEFT JOIN vehicles v ON b.vehicle_id = v.id LEFT JOIN users u ON b.renter_id = u.id ORDER BY b.created_at DESC LIMIT 10"
    ).unwrap();

    let recent_bookings: Vec<Value> = stmt
        .query_map([], |row| {
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
                "vehicle_name": format!("{} {}", row.get::<_, Option<String>>(11)?.unwrap_or_default(), row.get::<_, Option<String>>(12)?.unwrap_or_default()),
                "renter_name": row.get::<_, Option<String>>(13)?
            }))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    (
        StatusCode::OK,
        Json(json!({
            "total_users": total_users,
            "total_vehicles": total_vehicles,
            "total_bookings": total_bookings,
            "total_revenue": total_revenue,
            "recent_bookings": recent_bookings
        })),
    )
}

pub async fn list_users(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
) -> (StatusCode, Json<Value>) {
    if claims.role != "admin" {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Admin access required" })),
        );
    }

    let db = state.db.lock().unwrap();

    let mut stmt = db
        .prepare("SELECT id, phone, name, email, role, lang_pref, avatar_url, created_at FROM users ORDER BY created_at DESC")
        .unwrap();

    let users: Vec<Value> = stmt
        .query_map([], |row| {
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
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    (StatusCode::OK, Json(json!({ "users": users })))
}

pub async fn list_all_bookings(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
) -> (StatusCode, Json<Value>) {
    if claims.role != "admin" {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Admin access required" })),
        );
    }

    let db = state.db.lock().unwrap();

    let mut stmt = db.prepare(
        "SELECT b.id, b.renter_id, b.vehicle_id, b.driver_id, b.start_time, b.end_time, b.total_amount, b.payment_method, b.status, b.carbon_saved_kg, b.created_at, v.make, v.model, v.type, u.name as renter_name, u.phone as renter_phone FROM bookings b LEFT JOIN vehicles v ON b.vehicle_id = v.id LEFT JOIN users u ON b.renter_id = u.id ORDER BY b.created_at DESC"
    ).unwrap();

    let bookings: Vec<Value> = stmt
        .query_map([], |row| {
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
                    "type": row.get::<_, Option<String>>(13)?
                },
                "renter": {
                    "name": row.get::<_, Option<String>>(14)?,
                    "phone": row.get::<_, Option<String>>(15)?
                }
            }))
        })
        .unwrap()
        .filter_map(|r| r.ok())
        .collect();

    (StatusCode::OK, Json(json!({ "bookings": bookings })))
}
