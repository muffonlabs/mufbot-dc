#[poise::command(
    slash_command,
    prefix_command
)]

pub async fn shutdown(
    ctx: crate::discord::commands::Context<'_>
) -> Result<
    (),
    crate::discord::commands::Error
> {
    let guild = ctx
        .http()
        .get_guild(
            crate::env::GUILD_ID
                .as_str()
                .parse()
                .unwrap()
        )
        .await?;

    let role = guild
        .roles
        .get(
            &crate::env::ROLE_ID
                .as_str()
                .parse()
                .unwrap()
        )
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

    ctx.say("Shutting down...")
        .await?;

    ctx.framework()
        .shard_manager()
        .shutdown_all()
        .await;

    Ok(())
}
