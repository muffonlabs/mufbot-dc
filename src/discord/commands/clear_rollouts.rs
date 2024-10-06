#[poise::command(
    slash_command,
    prefix_command
)]

pub async fn clear_rollouts(
    ctx: crate::discord::commands::Context<'_>
) -> Result<
    (),
    crate::discord::commands::Error
> {
    // Auth check
    let guild_id = std::env::var(
        "MUFFON_GUILD_ID"
    )
    .expect("missing MUFFON_GUILD_ID");

    let role_id = std::env::var(
        "ROLLOUT_GROUP_ID"
    )
    .expect("missing ROLLOUT_GROUP_ID");

    let guild = ctx
        .http()
        .get_guild(
            guild_id.parse().unwrap()
        )
        .await?;

    let role = guild
        .roles
        .get(&role_id.parse().unwrap())
        .unwrap();

    if !ctx
        .author()
        .has_role(
            ctx.http(),
            &guild,
            role
        )
        .await?
    {
        ctx.say("You don't have permission to use this command")
            .await?;

        return Ok(());
    }

    let build_queue = ctx
        .data()
        .build_queue
        .lock()
        .await;

    build_queue
        .clear_queue()
        .expect(
            "failed to clear queue"
        );

    drop(build_queue);

    let response = "All rollouts have been cleared";

    ctx.say(response).await?;

    Ok(())
}
