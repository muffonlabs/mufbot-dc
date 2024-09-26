use sqlite::Statement;

pub const SQL_CMD: &str = "INSERT INTO rollout (version, status, approvals, rejections, creator, created_at) VALUES (?, ?, ?, ?, ?, ?)";

pub fn default_binds(
    stmt: &mut Statement<'_>,
    version: &str,
    creator_id: u64
) -> Result<
    (),
    Box<dyn std::error::Error>
> {

    stmt.bind((1, version))?;

    // pending at the time of creation
    // because we don't have approvals
    // yet
    stmt.bind((2, "pending"))?;

    stmt.bind((3, ""))?;

    stmt.bind((4, ""))?;

    stmt.bind((5, creator_id as i64))?;

    stmt.bind((
        6,
        chrono::Local::now()
            .to_string()
            .as_str()
    ))?;

    stmt.next()?;

    Ok(())
}
