use anyhow::Result;
use chrono::{TimeZone, Utc};
use rusqlite::{params, Connection, ToSql};

use crate::models::{Device, DeviceState, Metrics};

pub fn get_connection() -> rusqlite::Result<Connection> {
    Connection::open("./db/database.db3")
}

pub fn migrate(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        CREATE TABLE devices (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            codename TEXT NOT NULL,
            model TEXT NOT NULL,
            metrics TEXT NOT NULL,
            last_updated INTEGER DEFAULT 0
        )
    ",
        [],
    )?;
    Ok(())
}

pub fn create_device(conn: &Connection, name: &str, codename: &str, model: &str) -> Result<()> {
    conn.execute(
        "
        INSERT INTO devices(name, codename, model, metrics)
        VALUES (?1, ?2, ?3, ?4, ?5)
    ",
        params![name, codename, model, String::new()],
    )?;
    Ok(())
}

pub fn get_devices(conn: &Connection) -> Result<Vec<Device>> {
    let mut query = conn.prepare(
        "
        SELECT name, codename, model, metrics, last_updated
        FROM users
    ",
    )?;

    let res = query.query_map([], |row| {
        // find time since last updated
        let now = Utc::now();
        let last_updated = Utc.timestamp(row.get(4)?, 0);
        let diff = now - last_updated;

        // attempt to deserialize metrics
        let metrics_str: String = row.get(3)?;
        // TODO DANGER
        let metrics = serde_json::from_str(&metrics_str).unwrap();

        Ok(Device {
            name: row.get(0)?,
            codename: row.get(1)?,
            model: row.get(2)?,
            state: DeviceState::Online,
            metrics,
        })
    })?;
    Ok(vec![])
}

pub fn update_metrics(conn: &Connection, id: u32, metrics: &Metrics) -> Result<()> {
    let metrics_string = serde_json::to_string(metrics)?;
    let last_updated = Utc::now().timestamp();
    conn.execute(
        "
        UPDATE devices
        SET metrics = ?1, last_updated = ?2
        WHERE id = ?3
    ",
        params![metrics_string, last_updated, id],
    )?;
    Ok(())
}
