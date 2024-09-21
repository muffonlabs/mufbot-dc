use once_cell::sync::Lazy;
use std::env;

// Once cell makes sure a value is initialized only once at runtime,
// improving performance by avoiding redundant computations and
// multiple reads of the same environment variable.

pub static DISCORD_TOKEN: Lazy<String> =
    Lazy::new(|| {

        env::var("DISCORD_TOKEN")
            .expect(
                "missing DISCORD_TOKEN",
            )
    });

pub static GUILD_ID: Lazy<String> =
    Lazy::new(|| {

        env::var("MUFFON_GUILD_ID")
            .expect(
            "missing MUFFON_GUILD_ID",
        )
    });

// Define role_id as a lazy static value
pub static ROLE_ID: Lazy<String> =
    Lazy::new(|| {

        env::var("ROLLOUT_GROUP_ID")
            .expect(
            "missing ROLLOUT_GROUP_ID",
        )
    });
