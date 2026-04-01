use rusqlite::{Connection, Error};

pub fn init_databse() -> Result<Connection, Error> {
    let conn = Connection::open("vault.db").expect("Vault not found.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS services (
            id INTEGER PRIMARY KEY,
            service TEXT,
            url TEXT
        )", 
        [],
    ).expect("Failed to create database.");

    Ok(conn)
}
