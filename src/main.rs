use std::sync::Arc;

use dotenv::dotenv;
use poise::{
    framework,
    serenity_prelude::{self, futures::lock::Mutex},
};

mod db;
mod discord;

#[tokio::main]

async fn main() {

    dotenv().ok();

    let build_queue = Arc::new(Mutex::new(
        db::rollout::BuildQueue::new("muffon.db").expect("Failed to create BuildQueue"),
    ));

    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");

    println!("starting client");

    let intents = serenity_prelude::GatewayIntents::non_privileged();

    let framework = framework::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                discord::commands::age::age(),
                discord::commands::rollout::create_rollout(),
                discord::commands::buildlist::buildlist(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {

            Box::pin(async move {

                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(discord::commands::Data {
                    build_queue: build_queue.clone(),
                })
            })
        })
        .build();

    let client = serenity_prelude::Client::builder(token, intents)
        .framework(framework)
        .await;

    client
        .unwrap()
        .start()
        .await
        .expect("failed to start client");

    println!("client started");
}
