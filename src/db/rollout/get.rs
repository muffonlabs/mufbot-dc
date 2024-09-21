use sqlite::{State, Statement};

pub const SQL_CMD: &str = "SELECT * FROM rollout";

pub fn get_rollouts(stmt: &mut Statement<'_>) -> Vec<String> {

    let mut rollouts = vec![];

    while let Ok(State::Row) = stmt.next() {

        let version: String = stmt.read(0).expect("failed to read version");

        let status: String = stmt.read(1).expect("failed to read status");

        let approvals: i64 = stmt.read(2).expect("failed to read approvals");

        let rejections: i64 = stmt.read(3).expect("failed to read rejections");

        let created_at: String = stmt.read(4).expect("failed to read created_at");

        rollouts.push(format!(
            "{} {} {} {} {}",
            version, status, approvals, rejections, created_at
        ));
    }

    rollouts
}
