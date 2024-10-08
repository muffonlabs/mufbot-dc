use once_cell::sync::Lazy;
use std::env;

// Once cell makes sure a value is initialized only once at runtime,
// improving performance by avoiding redundant computations and
// multiple reads of the same environment variable.

pub static DISCORD_TOKEN: Lazy<String> =
    Lazy::new(|| {
        env::var("DISCORD_TOKEN")
            .expect(
                "missing DISCORD_TOKEN"
            )
    });

pub static GUILD_ID: Lazy<String> =
    Lazy::new(|| {
        env::var("MUFFON_GUILD_ID")
            .expect(
            "missing MUFFON_GUILD_ID"
        )
    });

// Define role_id as a lazy static value
pub static ROLE_ID: Lazy<String> =
    Lazy::new(|| {
        env::var("ROLLOUT_GROUP_ID")
            .expect(
            "missing ROLLOUT_GROUP_ID"
        )
    });

pub static SERVICE_NAME: Lazy<String> =
    Lazy::new(|| {
        env::var("SERVICE_NAME").expect(
            "missing SERVICE_NAME"
        )
    });

pub static BOTS_CHANNEL_ID: Lazy<
    String
> = Lazy::new(|| {
    env::var("BOTS_CHANNEL_ID").expect(
        "missing BOTS_CHANNEL_ID"
    )
});

pub static GITHUB_TOKEN: Lazy<String> =
    Lazy::new(|| {
        env::var("GITHUB_TOKEN").expect(
            "missing GITHUB_TOKEN"
        )
    });

pub static GITHUB_WORKFLOW_URL: Lazy<
    String
> = Lazy::new(|| {
    env::var("GITHUB_WORKFLOW_URL")
        .expect(
        "missing GITHUB_WORKFLOW_URL"
    )
});
