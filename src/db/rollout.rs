use sqlite::Connection;
mod get;
mod new;
mod queue;

pub struct BuildQueue {
    conn: Connection,
}

impl BuildQueue {
    pub fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = new::create(db_path)?;
        Ok(Self { conn })
    }

    // inserts a new rollout request into the queue
    pub fn queue_rollout(&self, version: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut stmt = self.conn.prepare(queue::SQL_CMD)?;

        queue::default_binds(&mut stmt, version)
    }

    pub fn get_builds(&self) -> Vec<String> {
        let mut stmt = self
            .conn
            .prepare(get::SQL_CMD)
            .expect("failed to prepare statement");

        get::get_rollouts(&mut stmt)
    }
}
