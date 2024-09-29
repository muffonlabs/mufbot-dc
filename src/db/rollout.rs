use sqlite::Connection;

mod get;
mod new;
mod queue;

pub struct BuildQueue {
    conn: Connection
}

impl BuildQueue {
    pub fn new(
        db_path: &str
    ) -> Result<
        Self,
        Box<dyn std::error::Error>
    > {

        let conn =
            new::create(db_path)?;

        Ok(Self { conn })
    }

    // inserts a new rollout request
    // into the queue
    pub fn queue_rollout(
        &self,
        version: &str,
        creator_id: u64
    ) -> Result<
        (),
        Box<dyn std::error::Error>
    > {

        let mut stmt = self
            .conn
            .prepare(queue::SQL_CMD)?;

        queue::default_binds(
            &mut stmt, version,
            creator_id
        )
    }

    pub fn get_builds(
        &self
    ) -> Vec<String> {

        let mut stmt = self
            .conn
            .prepare(get::SQL_CMD)
            .expect("failed to prepare statement");

        get::get_rollouts(&mut stmt)
    }

    pub fn get_rollout(
        &self,
        version: &str
    ) -> Option<Rollout> {

        let mut stmt = self
            .conn
            .prepare("SELECT * FROM rollout WHERE version = ?")
            .expect("failed to prepare statement");

        stmt.bind((1, version))
            .expect(
            "failed to bind version"
        );

        get::get_rollout(
            &mut stmt, version
        )
    }

    pub fn update_rollout(
        &self,
        rollout: &Rollout
    ) -> Result<
        (),
        Box<dyn std::error::Error>
    > {

        let approval_str = rollout
            .approvals
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let rejection_str = rollout
            .rejections
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let mut stmt = self
            .conn
            .prepare("UPDATE rollout SET status = ?, approvals = ?, rejections = ? WHERE version = ?")?;

        stmt.bind((
            1,
            rollout.status.as_str()
        ))?;

        stmt.bind((
            2,
            approval_str.as_str()
        ))?;

        stmt.bind((
            3,
            rejection_str.as_str()
        ))?;

        stmt.bind((
            4,
            rollout.version.as_str()
        ))?;

        stmt.next()?;

        Ok(())
    }

    pub fn approve_rollout(
        &self,
        version: &str,
        user_id: u64
    ) -> Result<
        bool,
        Box<dyn std::error::Error>
    > {

        let mut rollout = self
            .get_rollout(version)
            .expect("build not found");

        if rollout.status != "pending" {

            return Ok(false);
        }

        if rollout.creator == user_id {

            return Ok(false);
        }

        if rollout
            .rejections
            .contains(&user_id)
        {

            return Ok(false);
        }

        if rollout
            .approvals
            .contains(&user_id)
        {

            Ok(false)
        } else {

            rollout
                .approvals
                .push(user_id);

            if rollout.approvals.len()
                >= 2
                && rollout
                    .rejections
                    .is_empty()
            {

                rollout.status =
                    "approved"
                        .to_string();
            }

            self.update_rollout(
                &rollout
            )?;

            Ok(true)
        }
    }

    pub fn reject_rollout(
        &self,
        version: &str,
        user_id: u64
    ) -> Result<
        bool,
        Box<dyn std::error::Error>
    > {

        let mut rollout = self
            .get_rollout(version)
            .expect("build not found");

        if rollout.status != "pending" {

            return Ok(false);
        }

        if rollout.creator == user_id {

            return Ok(false);
        }

        if rollout
            .approvals
            .contains(&user_id)
        {

            return Ok(false);
        }

        if rollout
            .rejections
            .contains(&user_id)
        {

            Ok(false)
        } else {

            rollout
                .rejections
                .push(user_id);

            if !rollout
                .rejections
                .is_empty()
            {

                rollout.status =
                    "rejected"
                        .to_string();
            }

            self.update_rollout(
                &rollout
            )?;

            Ok(true)
        }
    }
}

pub struct Rollout {
    pub version: String,
    pub status: String,
    pub approvals: Vec<u64>,
    pub rejections: Vec<u64>,
    pub creator: u64,
    #[allow(dead_code)]
    pub created_at: String
}

// temporary implementation - so that there is no warning
impl Rollout {
    #[allow(dead_code)]

    pub fn get_created_at(
        &self
    ) -> &str {

        &self.created_at
    }
}
