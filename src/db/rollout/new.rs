use sqlite::Connection;
pub fn create(db_path: &str) -> Result<Connection, Box<dyn std::error::Error>> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS rollout (
            version TEXT PRIMARY KEY,
            status TEXT NOT NULL,
            approvals INTEGER NOT NULL,
            rejections INTEGER NOT NULL,
            created_at TEXT NOT NULL
        )",
    )?;
    Ok(conn)
}
