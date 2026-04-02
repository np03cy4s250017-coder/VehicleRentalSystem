use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use uuid::Uuid;

use crate::middleware::auth::AuthUser;
use crate::models::vehicle::{CreateVehicleRequest, UpdateVehicleRequest, VehicleQuery};
use crate::AppState;

fn row_to_vehicle(row: &rusqlite::Row) -> rusqlite::Result<Value> {
    let is_ev: i32 = row.get(6)?;
    let available: i32 = row.get(18)?;
    let features_str: String = row.get(19)?;
    let features: Value = serde_json::from_str(&features_str).unwrap_or(json!([]));

    Ok(json!({
        "id": row.get::<_, String>(0)?,
        "owner_id": row.get::<_, String>(1)?,
        "type": row.get::<_, String>(2)?,
        "make": row.get::<_, String>(3)?,
        "model": row.get::<_, String>(4)?,
        "year": row.get::<_, Option<i32>>(5)?,
        "is_ev": is_ev == 1,
        "ev_range_km": row.get::<_, Option<i32>>(7)?,
        "plate_no": row.get::<_, String>(8)?,
        "listing_type": row.get::<_, String>(9)?,
        "hourly_rate": row.get::<_, Option<f64>>(10)?,
        "daily_rate": row.get::<_, Option<f64>>(11)?,
        "description": row.get::<_, String>(12)?,
        "image_url": row.get::<_, String>(13)?,
        "location_name": row.get::<_, String>(14)?,
        "latitude": row.get::<_, f64>(15)?,
        "longitude": row.get::<_, f64>(16)?,
        "verified_at": row.get::<_, Option<String>>(17)?,
        "available": available == 1,
        "features": features,
        "created_at": row.get::<_, String>(20)?,
    }))
}

pub async fn list_vehicles(
    State(state): State<Arc<AppState>>,
    Query(query): Query<VehicleQuery>,
) -> (StatusCode, Json<Value>) {
    let db = state.db.lock().unwrap();

    let mut sql = String::from(
        "SELECT id, owner_id, type, make, model, year, is_ev, ev_range_km, plate_no, listing_type, hourly_rate, daily_rate, description, image_url, location_name, latitude, longitude, verified_at, available, features, created_at FROM vehicles WHERE 1=1"
    );
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    let mut param_idx = 1;

    if let Some(ref vtype) = query.vehicle_type {
        sql.push_str(&format!(" AND type = ?{}", param_idx));
        params.push(Box::new(vtype.clone()));
        param_idx += 1;
    }

    if let Some(is_ev) = query.is_ev {
        sql.push_str(&format!(" AND is_ev = ?{}", param_idx));
        params.push(Box::new(if is_ev { 1i32 } else { 0i32 }));
        param_idx += 1;
    }

    if let Some(min_price) = query.min_price {
        sql.push_str(&format!(" AND daily_rate >= ?{}", param_idx));
        params.push(Box::new(min_price));
        param_idx += 1;
    }

    if let Some(max_price) = query.max_price {
        sql.push_str(&format!(" AND daily_rate <= ?{}", param_idx));
        params.push(Box::new(max_price));
        param_idx += 1;
    }

    if let Some(ref search) = query.search {
        sql.push_str(&format!(
            " AND (make LIKE ?{p} OR model LIKE ?{p} OR description LIKE ?{p} OR location_name LIKE ?{p})",
            p = param_idx
        ));
        params.push(Box::new(format!("%{}%", search)));
        param_idx += 1;
    }

    if let Some(avail) = query.available {
        sql.push_str(&format!(" AND available = ?{}", param_idx));
        params.push(Box::new(if avail { 1i32 } else { 0i32 }));
        let _ = param_idx;
    }

    sql.push_str(" ORDER BY created_at DESC");

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let mut stmt = match db.prepare(&sql) {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Query error: {}", e) })),
            );
        }
    };

    let vehicles: Vec<Value> = match stmt.query_map(param_refs.as_slice(), |row| row_to_vehicle(row)) {
        Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Query error: {}", e) })),
            );
        }
    };

    (StatusCode::OK, Json(json!({ "vehicles": vehicles })))
}

pub async fn get_vehicle(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    let db = state.db.lock().unwrap();

    let result = db.query_row(
        "SELECT v.id, v.owner_id, v.type, v.make, v.model, v.year, v.is_ev, v.ev_range_km, v.plate_no, v.listing_type, v.hourly_rate, v.daily_rate, v.description, v.image_url, v.location_name, v.latitude, v.longitude, v.verified_at, v.available, v.features, v.created_at, u.name as owner_name, u.phone as owner_phone FROM vehicles v LEFT JOIN users u ON v.owner_id = u.id WHERE v.id = ?1",
        rusqlite::params![id],
        |row| {
            let is_ev: i32 = row.get(6)?;
            let available: i32 = row.get(18)?;
            let features_str: String = row.get(19)?;
            let features: Value = serde_json::from_str(&features_str).unwrap_or(json!([]));

            Ok(json!({
                "id": row.get::<_, String>(0)?,
                "owner_id": row.get::<_, String>(1)?,
                "type": row.get::<_, String>(2)?,
                "make": row.get::<_, String>(3)?,
                "model": row.get::<_, String>(4)?,
                "year": row.get::<_, Option<i32>>(5)?,
                "is_ev": is_ev == 1,
                "ev_range_km": row.get::<_, Option<i32>>(7)?,
                "plate_no": row.get::<_, String>(8)?,
                "listing_type": row.get::<_, String>(9)?,
                "hourly_rate": row.get::<_, Option<f64>>(10)?,
                "daily_rate": row.get::<_, Option<f64>>(11)?,
                "description": row.get::<_, String>(12)?,
                "image_url": row.get::<_, String>(13)?,
                "location_name": row.get::<_, String>(14)?,
                "latitude": row.get::<_, f64>(15)?,
                "longitude": row.get::<_, f64>(16)?,
                "verified_at": row.get::<_, Option<String>>(17)?,
                "available": available == 1,
                "features": features,
                "created_at": row.get::<_, String>(20)?,
                "owner": {
                    "name": row.get::<_, Option<String>>(21)?,
                    "phone": row.get::<_, Option<String>>(22)?
                }
            }))
        },
    );

    match result {
        Ok(vehicle) => (StatusCode::OK, Json(json!({ "vehicle": vehicle }))),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Vehicle not found" })),
        ),
    }
}

pub async fn create_vehicle(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
    Json(payload): Json<CreateVehicleRequest>,
) -> (StatusCode, Json<Value>) {
    if claims.role != "owner" && claims.role != "admin" {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "Only owners and admins can create vehicle listings" })),
        );
    }

    let id = Uuid::new_v4().to_string();
    let is_ev = payload.is_ev.unwrap_or(true);
    let listing_type = payload.listing_type.unwrap_or_else(|| "p2p".to_string());
    let description = payload.description.unwrap_or_default();
    let image_url = payload.image_url.unwrap_or_default();
    let location_name = payload.location_name.unwrap_or_else(|| "Kathmandu".to_string());
    let latitude = payload.latitude.unwrap_or(27.7172);
    let longitude = payload.longitude.unwrap_or(85.324);
    let features = payload.features.unwrap_or(json!([])).to_string();

    let db = state.db.lock().unwrap();
    let result = db.execute(
        "INSERT INTO vehicles (id, owner_id, type, make, model, year, is_ev, ev_range_km, plate_no, listing_type, hourly_rate, daily_rate, description, image_url, location_name, latitude, longitude, available, features) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,1,?18)",
        rusqlite::params![
            id,
            claims.user_id,
            payload.vehicle_type,
            payload.make,
            payload.model,
            payload.year,
            if is_ev { 1 } else { 0 },
            payload.ev_range_km,
            payload.plate_no,
            listing_type,
            payload.hourly_rate,
            payload.daily_rate,
            description,
            image_url,
            location_name,
            latitude,
            longitude,
            features,
        ],
    );

    match result {
        Ok(_) => {
            // Fetch the created vehicle
            let vehicle = db.query_row(
                "SELECT id, owner_id, type, make, model, year, is_ev, ev_range_km, plate_no, listing_type, hourly_rate, daily_rate, description, image_url, location_name, latitude, longitude, verified_at, available, features, created_at FROM vehicles WHERE id = ?1",
                rusqlite::params![id],
                |row| row_to_vehicle(row),
            );
            match vehicle {
                Ok(v) => (StatusCode::CREATED, Json(json!({ "vehicle": v }))),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("Created but failed to fetch: {}", e) })),
                ),
            }
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Failed to create vehicle: {}", e) })),
        ),
    }
}

pub async fn update_vehicle(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
    Path(id): Path<String>,
    Json(payload): Json<UpdateVehicleRequest>,
) -> (StatusCode, Json<Value>) {
    let db = state.db.lock().unwrap();

    // Check ownership
    let owner_id: Result<String, _> = db.query_row(
        "SELECT owner_id FROM vehicles WHERE id = ?1",
        rusqlite::params![id],
        |row| row.get(0),
    );

    match owner_id {
        Ok(oid) => {
            if oid != claims.user_id && claims.role != "admin" {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({ "error": "You can only update your own vehicles" })),
                );
            }
        }
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Vehicle not found" })),
            );
        }
    }

    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    let mut idx = 1;

    macro_rules! add_field {
        ($field:expr, $col:expr) => {
            if let Some(ref val) = $field {
                sets.push(format!("{} = ?{}", $col, idx));
                params.push(Box::new(val.clone()));
                idx += 1;
            }
        };
    }

    add_field!(payload.vehicle_type, "type");
    add_field!(payload.make, "make");
    add_field!(payload.model, "model");
    add_field!(payload.year, "year");
    add_field!(payload.plate_no, "plate_no");
    add_field!(payload.listing_type, "listing_type");
    add_field!(payload.hourly_rate, "hourly_rate");
    add_field!(payload.daily_rate, "daily_rate");
    add_field!(payload.description, "description");
    add_field!(payload.image_url, "image_url");
    add_field!(payload.location_name, "location_name");
    add_field!(payload.latitude, "latitude");
    add_field!(payload.longitude, "longitude");
    add_field!(payload.ev_range_km, "ev_range_km");

    if let Some(is_ev) = payload.is_ev {
        sets.push(format!("is_ev = ?{}", idx));
        params.push(Box::new(if is_ev { 1i32 } else { 0i32 }));
        idx += 1;
    }

    if let Some(avail) = payload.available {
        sets.push(format!("available = ?{}", idx));
        params.push(Box::new(if avail { 1i32 } else { 0i32 }));
        idx += 1;
    }

    if let Some(ref features) = payload.features {
        sets.push(format!("features = ?{}", idx));
        params.push(Box::new(features.to_string()));
        idx += 1;
    }

    if sets.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "No fields to update" })),
        );
    }

    let sql = format!(
        "UPDATE vehicles SET {} WHERE id = ?{}",
        sets.join(", "),
        idx
    );
    params.push(Box::new(id.clone()));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    match db.execute(&sql, param_refs.as_slice()) {
        Ok(_) => {
            let vehicle = db.query_row(
                "SELECT id, owner_id, type, make, model, year, is_ev, ev_range_km, plate_no, listing_type, hourly_rate, daily_rate, description, image_url, location_name, latitude, longitude, verified_at, available, features, created_at FROM vehicles WHERE id = ?1",
                rusqlite::params![id],
                |row| row_to_vehicle(row),
            );
            match vehicle {
                Ok(v) => (StatusCode::OK, Json(json!({ "vehicle": v }))),
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": format!("Updated but failed to fetch: {}", e) })),
                ),
            }
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": format!("Failed to update vehicle: {}", e) })),
        ),
    }
}

pub async fn delete_vehicle(
    State(state): State<Arc<AppState>>,
    AuthUser(claims): AuthUser,
    Path(id): Path<String>,
) -> (StatusCode, Json<Value>) {
    let db = state.db.lock().unwrap();

    // Check ownership
    let owner_id: Result<String, _> = db.query_row(
        "SELECT owner_id FROM vehicles WHERE id = ?1",
        rusqlite::params![id],
        |row| row.get(0),
    );

    match owner_id {
        Ok(oid) => {
            if oid != claims.user_id && claims.role != "admin" {
                return (
                    StatusCode::FORBIDDEN,
                    Json(json!({ "error": "You can only delete your own vehicles" })),
                );
            }
        }
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Vehicle not found" })),
            );
        }
    }

    match db.execute("DELETE FROM vehicles WHERE id = ?1", rusqlite::params![id]) {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "success": true, "message": "Vehicle deleted successfully" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to delete vehicle: {}", e) })),
        ),
    }
}
