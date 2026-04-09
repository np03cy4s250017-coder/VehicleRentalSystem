use serde::Deserialize;

/// eSewa payment verification request
/// After eSewa redirects back with transaction details
#[derive(Debug, Deserialize)]
pub struct EsewaVerifyRequest {
    pub booking_id: String,
    /// eSewa reference ID (rid) from redirect callback
    pub reference_id: Option<String>,
    /// Product ID (pid) used in the eSewa payment form
    pub product_id: Option<String>,
    /// Transaction amount in NPR
    pub amount: Option<f64>,
}

/// Khalti payment verification request
/// After client-side Khalti SDK returns a payment token
#[derive(Debug, Deserialize)]
pub struct KhaltiVerifyRequest {
    pub booking_id: String,
    /// Khalti payment token from client SDK
    pub token: String,
    /// Amount in paisa (1 NPR = 100 paisa) or NPR
    pub amount: Option<f64>,
}

/// Vehicle search request body matching website API
#[derive(Debug, Deserialize)]
pub struct VehicleSearchRequest {
    /// Vehicle type filter
    #[serde(rename = "type")]
    pub vehicle_type: Option<String>,
    /// Minimum daily rate
    pub min_rate: Option<f64>,
    /// Maximum daily rate
    pub max_rate: Option<f64>,
    /// EV range minimum in km
    pub min_range: Option<i32>,
    /// Location name or area
    pub location: Option<String>,
    /// Text search across make, model, description
    pub query: Option<String>,
    /// EV only filter
    pub is_ev: Option<bool>,
    /// Start date for availability check
    pub start_date: Option<String>,
    /// End date for availability check
    pub end_date: Option<String>,
}
