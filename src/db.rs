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
    ).expect("Failed to create service table.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY,
            service_id INTEGER,
            username TEXT,
            email TEXT,
            password TEXT, 
            access_token TEXT, 
            last_change TEXT, 
            account_creation_date TEXT, 
            pin INTEGER,
            passcode TEXT,
        )", 
        [],
    ).expect("Failed to create accounts table.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS security_questions (
            id INTEGER PRIMARY KEY,
            account_id INTEGER,
            question TEXT,
            answer TEXT
        )", 
        [],
    ).expect("Failed to create security question table.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS shortcuts (
            id INTEGER PRIMARY KEY,
            account_id INTEGER,
            field TEXT,
            sequence TEXT
        )", 
        [],
    ).expect("Failed to create security question table.");

    Ok(conn)
}
