#[poise::command(
    slash_command,
    prefix_command
)]

pub async fn create_rollout(
    ctx: crate::discord::commands::Context<'_>,
    #[description = "Version to roll out"]
    version: String,
) -> Result<
    (),
    crate::discord::commands::Error,
> {

    let guild = ctx
        .http()
        .get_guild(
            crate::env::GUILD_ID
                .as_str()
                .parse()
                .unwrap(),
        )
        .await?;

    let role = guild
        .roles
        .get(
            &crate::env::ROLE_ID
                .as_str()
                .parse()
                .unwrap(),
        )
        .unwrap();

    if !ctx
        .author()
        .has_role(
            ctx.http(),
            &guild,
            role,
        )
        .await?
    {

        ctx.say("You don't have permission to use this command")
            .await?;

        return Ok(());
    }

    // Add rollout to queue
    let build_queue = ctx
        .data()
        .build_queue
        .lock()
        .await;

    build_queue
        .queue_rollout(&version)
        .expect(
            "failed to queue build",
        );

    drop(build_queue);

    let response = format!(
        "Rollout of version {} started",
        version
    );

    ctx.say(response).await?;

    Ok(())
}
