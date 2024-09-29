pub mod commands;
pub mod utils;

use poise::{
    framework,
    serenity_prelude::{
        self, futures::lock::Mutex
    }
};
use std::sync::Arc;

fn get_build_queue() -> Arc<
    Mutex<
        crate::db::rollout::BuildQueue
    >
> {

    Arc::new(Mutex::new(
        crate::db::rollout::BuildQueue::new("muffon.db")
            .expect("Failed to create BuildQueue"),
    ))
}

pub async fn initiate_bot() {

    let build_queue: Arc<Mutex<crate::db::rollout::BuildQueue>> = get_build_queue();

    println!("starting client");

    let intents = serenity_prelude::GatewayIntents::non_privileged();

    let framework = framework::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::age::age(),
                commands::rollout::create_rollout(),
                commands::buildlist::buildlist(),
                commands::shutdown::shutdown(),
                commands::restart::restart(),
                commands::version::version(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {

            Box::pin(async move {

                poise::builtins::register_globally(ctx,&framework.options().commands).await?;

                let channel_id = serenity_prelude::ChannelId::new(crate::env::BOTS_CHANNEL_ID.parse().unwrap());
                channel_id.say(&ctx.http, "mufbot is online!").await.unwrap();

                Ok(commands::Data {
                    build_queue: build_queue.clone(),
                })
            })
        })
        .build();

    let client = serenity_prelude::Client::builder(crate::env::DISCORD_TOKEN.as_str(), intents)
        .framework(framework)
        .await;

    client
        .unwrap()
        .start()
        .await
        .expect(
            "failed to start client"
        );

    println!("client started");
}
