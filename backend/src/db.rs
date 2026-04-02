use rusqlite::{Connection, Result};

pub fn initialize_db() -> Result<Connection> {
    let conn = Connection::open("yatrasathi.db")?;

    conn.execute_batch("PRAGMA journal_mode=WAL;")?;
    conn.execute_batch("PRAGMA foreign_keys=ON;")?;

    create_tables(&conn)?;
    seed_data(&conn)?;

    Ok(conn)
}

fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            phone TEXT UNIQUE NOT NULL,
            name TEXT DEFAULT '',
            email TEXT DEFAULT '',
            role TEXT DEFAULT 'renter' CHECK(role IN ('renter','driver','owner','admin')),
            lang_pref TEXT DEFAULT 'ne',
            avatar_url TEXT DEFAULT '',
            created_at TEXT DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS vehicles (
            id TEXT PRIMARY KEY,
            owner_id TEXT REFERENCES users(id),
            type TEXT NOT NULL CHECK(type IN ('car','suv','bike','scooter','jeep')),
            make TEXT NOT NULL,
            model TEXT NOT NULL,
            year INTEGER,
            is_ev INTEGER DEFAULT 1,
            ev_range_km INTEGER,
            plate_no TEXT UNIQUE,
            listing_type TEXT DEFAULT 'p2p' CHECK(listing_type IN ('p2p','fleet','hotel')),
            hourly_rate REAL,
            daily_rate REAL,
            description TEXT DEFAULT '',
            image_url TEXT DEFAULT '',
            location_name TEXT DEFAULT 'Kathmandu',
            latitude REAL DEFAULT 27.7172,
            longitude REAL DEFAULT 85.324,
            available INTEGER DEFAULT 1,
            features TEXT DEFAULT '[]',
            verified_at TEXT,
            created_at TEXT DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS bookings (
            id TEXT PRIMARY KEY,
            renter_id TEXT REFERENCES users(id),
            vehicle_id TEXT REFERENCES vehicles(id),
            driver_id TEXT,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL,
            total_amount REAL NOT NULL,
            payment_method TEXT DEFAULT 'esewa' CHECK(payment_method IN ('esewa','khalti','cash','connectips')),
            status TEXT DEFAULT 'pending' CHECK(status IN ('pending','confirmed','active','completed','cancelled')),
            carbon_saved_kg REAL DEFAULT 0,
            created_at TEXT DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS otp_codes (
            id TEXT PRIMARY KEY,
            phone TEXT NOT NULL,
            code TEXT NOT NULL,
            expires_at TEXT NOT NULL,
            used INTEGER DEFAULT 0
        );
        ",
    )?;

    Ok(())
}

fn seed_data(conn: &Connection) -> Result<()> {
    // Seed users
    conn.execute(
        "INSERT OR IGNORE INTO users (id, phone, name, role) VALUES (?1, ?2, ?3, ?4)",
        ("a1000001-0000-0000-0000-000000000001", "9841000001", "Rajesh Hamal", "admin"),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO users (id, phone, name, role) VALUES (?1, ?2, ?3, ?4)",
        ("a1000002-0000-0000-0000-000000000002", "9841000002", "Sita Sharma", "owner"),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO users (id, phone, name, role) VALUES (?1, ?2, ?3, ?4)",
        ("a1000003-0000-0000-0000-000000000003", "9841000003", "Bikram Thapa", "owner"),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO users (id, phone, name, role) VALUES (?1, ?2, ?3, ?4)",
        ("a1000004-0000-0000-0000-000000000004", "9841000004", "Kumar Tamang", "driver"),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO users (id, phone, name, role) VALUES (?1, ?2, ?3, ?4)",
        ("a1000005-0000-0000-0000-000000000005", "9841000005", "Maya Gurung", "renter"),
    )?;

    // Seed vehicles
    let owner1 = "a1000002-0000-0000-0000-000000000002";
    let owner2 = "a1000003-0000-0000-0000-000000000003";

    conn.execute(
        "INSERT OR IGNORE INTO vehicles (id, owner_id, type, make, model, year, is_ev, ev_range_km, plate_no, listing_type, hourly_rate, daily_rate, description, image_url, location_name, latitude, longitude, available, features) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19)",
        rusqlite::params![
            "v1000001-0000-0000-0000-000000000001", owner1, "suv", "BYD", "Atto 3", 2024, 1, 420,
            "Ba 1 Pa 2024", "p2p", 2500.0, 12000.0,
            "Premium electric SUV with fast charging capability",
            "https://images.unsplash.com/photo-1694404102669-1b4161814818?w=800&h=600&fit=crop",
            "Thamel", 27.7154, 85.3123, 1,
            r#"["AC","GPS","Bluetooth","Fast Charging"]"#
        ],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO vehicles (id, owner_id, type, make, model, year, is_ev, ev_range_km, plate_no, listing_type, hourly_rate, daily_rate, description, image_url, location_name, latitude, longitude, available, features) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19)",
        rusqlite::params![
            "v1000002-0000-0000-0000-000000000002", owner1, "suv", "Tata", "Nexon EV", 2024, 1, 312,
            "Ba 2 Pa 3045", "p2p", 2000.0, 10000.0,
            "Reliable electric SUV for city and highway",
            "https://images.unsplash.com/photo-1619767886558-efdc7b9af5a6?w=800&h=600&fit=crop",
            "Patan", 27.6727, 85.3240, 1,
            r#"["AC","GPS","Airbags"]"#
        ],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO vehicles (id, owner_id, type, make, model, year, is_ev, ev_range_km, plate_no, listing_type, hourly_rate, daily_rate, description, image_url, location_name, latitude, longitude, available, features) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19)",
        rusqlite::params![
            "v1000003-0000-0000-0000-000000000003", owner2, "car", "MG", "ZS EV", 2023, 1, 350,
            "Ba 1 Cha 5567", "p2p", 2200.0, 11000.0,
            "Stylish electric car with sunroof",
            "https://images.unsplash.com/photo-1560958089-b8a1929cea89?w=800&h=600&fit=crop",
            "Bouddha", 27.7215, 85.3620, 1,
            r#"["AC","GPS","Bluetooth","Sunroof"]"#
        ],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO vehicles (id, owner_id, type, make, model, year, is_ev, ev_range_km, plate_no, listing_type, hourly_rate, daily_rate, description, image_url, location_name, latitude, longitude, available, features) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19)",
        rusqlite::params![
            "v1000004-0000-0000-0000-000000000004", owner2, "suv", "Mahindra", "XUV400", 2024, 1, 456,
            "Ba 3 Pa 7788", "p2p", 1800.0, 9000.0,
            "Long range electric SUV with fast charging",
            "https://images.unsplash.com/photo-1669725083850-a3e20db34781?w=800&h=600&fit=crop",
            "Lalitpur", 27.6588, 85.3247, 1,
            r#"["AC","GPS","Fast Charging"]"#
        ],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO vehicles (id, owner_id, type, make, model, year, is_ev, ev_range_km, plate_no, listing_type, hourly_rate, daily_rate, description, image_url, location_name, latitude, longitude, available, features) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19)",
        rusqlite::params![
            "v1000005-0000-0000-0000-000000000005", owner1, "scooter", "NIU", "NQi GTS", 2023, 1, 80,
            "Ba 14 Pa 1122", "p2p", 300.0, 1500.0,
            "Nimble electric scooter for city commute",
            "https://images.unsplash.com/photo-1614165936528-af2006416b4b?w=800&h=600&fit=crop",
            "Thamel", 27.7154, 85.3123, 1,
            r#"["USB Charging","Digital Display"]"#
        ],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO vehicles (id, owner_id, type, make, model, year, is_ev, ev_range_km, plate_no, listing_type, hourly_rate, daily_rate, description, image_url, location_name, latitude, longitude, available, features) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19)",
        rusqlite::params![
            "v1000006-0000-0000-0000-000000000006", owner2, "scooter", "Honda", "Dio", 2023, 0,
            Option::<i32>::None,
            "Ba 14 Pa 3344", "p2p", 200.0, 1000.0,
            "Lightweight scooter perfect for short trips",
            "https://images.unsplash.com/photo-1558980664-769d59546b3d?w=800&h=600&fit=crop",
            "Bhaktapur", 27.6710, 85.4298, 1,
            r#"["Lightweight","Easy Parking"]"#
        ],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO vehicles (id, owner_id, type, make, model, year, is_ev, ev_range_km, plate_no, listing_type, hourly_rate, daily_rate, description, image_url, location_name, latitude, longitude, available, features) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19)",
        rusqlite::params![
            "v1000007-0000-0000-0000-000000000007", owner1, "bike", "Royal Enfield", "Himalayan", 2023, 0,
            Option::<i32>::None,
            "Ba 5 Pa 9900", "p2p", 800.0, 4000.0,
            "Adventure-ready bike for mountain roads",
            "https://images.unsplash.com/photo-1558981285-6f0c94958bb6?w=800&h=600&fit=crop",
            "Thamel", 27.7154, 85.3123, 1,
            r#"["Adventure Ready","Luggage Rack","GPS"]"#
        ],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO vehicles (id, owner_id, type, make, model, year, is_ev, ev_range_km, plate_no, listing_type, hourly_rate, daily_rate, description, image_url, location_name, latitude, longitude, available, features) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19)",
        rusqlite::params![
            "v1000008-0000-0000-0000-000000000008", owner2, "jeep", "Mahindra", "Bolero", 2022, 0,
            Option::<i32>::None,
            "Ba 2 Kha 4455", "p2p", 1500.0, 7000.0,
            "Rugged jeep for off-road adventures",
            "https://images.unsplash.com/photo-1519641471654-76ce0107ad1b?w=800&h=600&fit=crop",
            "Bhaktapur", 27.6710, 85.4298, 1,
            r#"["4WD","AC","7 Seater","Roof Rack"]"#
        ],
    )?;

    // Seed bookings
    let renter = "a1000005-0000-0000-0000-000000000005";

    conn.execute(
        "INSERT OR IGNORE INTO bookings (id, renter_id, vehicle_id, start_time, end_time, total_amount, payment_method, status, carbon_saved_kg) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
        rusqlite::params![
            "b1000001-0000-0000-0000-000000000001", renter,
            "v1000001-0000-0000-0000-000000000001",
            "2024-03-15T10:00:00", "2024-03-16T10:00:00",
            12000.0, "esewa", "pending", 8.5
        ],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO bookings (id, renter_id, vehicle_id, start_time, end_time, total_amount, payment_method, status, carbon_saved_kg) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
        rusqlite::params![
            "b1000002-0000-0000-0000-000000000002", renter,
            "v1000005-0000-0000-0000-000000000005",
            "2024-03-10T08:00:00", "2024-03-10T18:00:00",
            1500.0, "khalti", "confirmed", 1.2
        ],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO bookings (id, renter_id, vehicle_id, start_time, end_time, total_amount, payment_method, status, carbon_saved_kg) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
        rusqlite::params![
            "b1000003-0000-0000-0000-000000000003", renter,
            "v1000007-0000-0000-0000-000000000007",
            "2024-03-01T10:00:00", "2024-03-03T10:00:00",
            12000.0, "cash", "completed", 0.0
        ],
    )?;

    Ok(())
}
