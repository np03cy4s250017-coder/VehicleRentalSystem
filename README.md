# YatraSathi — Vehicle Rental System

A full-stack vehicle rental platform built for Semester 4 Collaborative Development coursework.

## Stack

- **Backend:** Rust (Axum) + SQLite + JWT auth
- **Frontend:** SvelteKit + Tailwind CSS
- **Payments:** eSewa & Khalti integration
- **SMS/OTP:** Sparrow SMS

## Features

- Phone + password authentication (Consumer, Owner, Driver, Admin roles)
- Admin 2FA (password + OTP)
- Vehicle search, booking, and management
- Admin dashboard for bookings, vehicles, and users
- Driver and bundle listings
- Reviews and ratings

## Running locally

```bash
docker-compose up
```

Or run backend and frontend separately — see `backend/` and `frontend/` directories.
