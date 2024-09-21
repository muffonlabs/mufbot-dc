pub mod age;
pub mod buildlist;
pub mod rollout;
use poise::serenity_prelude::futures::lock::Mutex;
use std::{any, sync::Arc};

pub struct Data {
    pub build_queue: Arc<Mutex<crate::db::rollout::BuildQueue>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
