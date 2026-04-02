use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Booking {
    pub id: String,
    pub renter_id: String,
    pub vehicle_id: String,
    pub driver_id: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub total_amount: f64,
    pub payment_method: String,
    pub status: String,
    pub carbon_saved_kg: f64,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateBookingRequest {
    pub vehicle_id: String,
    pub start_time: String,
    pub end_time: String,
    pub payment_method: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBookingStatusRequest {
    pub status: String,
}
