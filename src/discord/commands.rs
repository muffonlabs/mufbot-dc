pub mod age;
pub mod buildlist;
pub mod restart;
pub mod rollout;
pub mod shutdown;
pub mod version;

use poise::serenity_prelude::futures::lock::Mutex;
use std::sync::Arc;

pub struct Data {
    pub build_queue: Arc<Mutex<crate::db::rollout::BuildQueue>>,
}

pub type Error = Box<
    dyn std::error::Error + Send + Sync
>;

pub type Context<'a> =
    poise::Context<'a, Data, Error>;
