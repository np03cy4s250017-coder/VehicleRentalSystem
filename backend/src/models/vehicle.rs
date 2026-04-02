use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: String,
    pub owner_id: String,
    #[serde(rename = "type")]
    pub vehicle_type: String,
    pub make: String,
    pub model: String,
    pub year: Option<i32>,
    pub is_ev: bool,
    pub ev_range_km: Option<i32>,
    pub plate_no: String,
    pub listing_type: String,
    pub hourly_rate: Option<f64>,
    pub daily_rate: Option<f64>,
    pub description: String,
    pub image_url: String,
    pub location_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub available: bool,
    pub features: serde_json::Value,
    pub verified_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateVehicleRequest {
    #[serde(rename = "type")]
    pub vehicle_type: String,
    pub make: String,
    pub model: String,
    pub year: Option<i32>,
    pub is_ev: Option<bool>,
    pub ev_range_km: Option<i32>,
    pub plate_no: String,
    pub listing_type: Option<String>,
    pub hourly_rate: Option<f64>,
    pub daily_rate: Option<f64>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub location_name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub features: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVehicleRequest {
    #[serde(rename = "type")]
    pub vehicle_type: Option<String>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<i32>,
    pub is_ev: Option<bool>,
    pub ev_range_km: Option<i32>,
    pub plate_no: Option<String>,
    pub listing_type: Option<String>,
    pub hourly_rate: Option<f64>,
    pub daily_rate: Option<f64>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub location_name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub available: Option<bool>,
    pub features: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct VehicleQuery {
    #[serde(rename = "type")]
    pub vehicle_type: Option<String>,
    pub is_ev: Option<bool>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub search: Option<String>,
    pub available: Option<bool>,
}
