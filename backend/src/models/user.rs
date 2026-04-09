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

// Legacy OTP requests (kept for backward compat)
#[derive(Debug, Deserialize)]
pub struct OtpSendRequest {
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct OtpVerifyRequest {
    pub phone: String,
    pub otp: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminLoginRequest {
    pub username: String,
    pub password: String,
}

// New auth: phone + password login
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub phone: String,
    pub password: String,
}

// New auth: registration (consumer/owner/driver only)
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub phone: String,
    pub password: String,
    pub name: String,
    pub role: String, // consumer, owner, driver
}

// Verify registration OTP
#[derive(Debug, Deserialize)]
pub struct RegisterVerifyRequest {
    pub phone: String,
    pub otp: String,
    pub name: String,
    pub password: String,
    pub role: String,
}

// Admin 2FA: verify OTP after password login
#[derive(Debug, Deserialize)]
pub struct AdminOtpVerifyRequest {
    pub phone: String,
    pub otp: String,
}

// Password reset: request OTP
#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub phone: String,
}

// Password reset: verify OTP and set new password
#[derive(Debug, Deserialize)]
pub struct ResetPasswordVerifyRequest {
    pub phone: String,
    pub otp: String,
    pub new_password: String,
}
