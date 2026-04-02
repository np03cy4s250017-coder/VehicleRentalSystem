use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub phone: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub lang_pref: String,
    pub avatar_url: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct OtpSendRequest {
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct OtpVerifyRequest {
    pub phone: String,
    pub otp: String,
}
