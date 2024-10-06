use sqlite::{State, Statement};

use super::Rollout;

pub const SQL_CMD: &str =
    "SELECT * FROM rollout";

pub fn get_rollouts(
    stmt: &mut Statement<'_>
) -> Vec<String> {
    let mut rollouts = vec![];

    while let Ok(State::Row) =
        stmt.next()
    {
        let version: String = stmt
            .read(0)
            .expect("failed to read version");

        let status: String =
            stmt.read(1).expect(
                "failed to read status"
            );

        let approvals: Vec<u64> = stmt
            .read(2)
            .unwrap_or("0".to_string())
            .split(',')
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.parse().unwrap_or(0)
            })
            .collect();

        let rejections: Vec<u64> = stmt
            .read(3)
            .unwrap_or("0".to_string())
            .split(',')
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.parse().unwrap_or(0)
            })
            .collect();

        let creator: i64 = stmt
            .read(4)
            .expect("failed to read creator");

        let created_at: String = stmt
            .read(5)
            .expect("failed to read created_at");

        rollouts.push(format!(
            "{} {} {:?} {:?} {} {}",
            version,
            status,
            approvals,
            rejections,
            creator,
            created_at
        ));
    }

    rollouts
}

pub fn get_rollout(
    stmt: &mut Statement<'_>,
    version: &str
) -> Option<Rollout> {
    while let Ok(State::Row) =
        stmt.next()
    {
        let version_db: String = stmt
            .read(0)
            .expect("failed to read version");

        if version_db == version {
            let status: String = stmt
                .read(1)
                .expect("failed to read status");

            let approvals: Vec<u64> =
                stmt.read(2)
                    .unwrap_or(
                        "0".to_string()
                    )
                    .split(',')
                    .filter(|x| {
                        !x.is_empty()
                    })
                    .map(|x| {
                        x.parse()
                            .unwrap_or(
                                0
                            )
                    })
                    .collect();

            let rejections: Vec<u64> =
                stmt.read(3)
                    .unwrap_or(
                        "0".to_string()
                    )
                    .split(',')
                    .filter(|x| {
                        !x.is_empty()
                    })
                    .map(|x| {
                        x.parse()
                            .unwrap_or(
                                0
                            )
                    })
                    .collect();

            let creator: u64 = stmt
                .read(4)
                .unwrap_or("0".to_string()) // necessary for trait bound
                .parse()
                .expect("failed to read creator");

            let created_at: String = stmt
                .read(5)
                .expect("failed to read created_at");

            return Some(Rollout {
                version: version
                    .to_string(),
                status,
                approvals,
                rejections,
                creator,
                created_at
            });
        }
    }

    None
}
