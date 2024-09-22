#[poise::command(
    slash_command,
    prefix_command
)]

pub async fn restart(
    ctx: crate::discord::commands::Context<'_>,
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

    ctx.say("Restarting...")
        .await?;

    let cmd =
        std::process::Command::new(
            "systemctl",
        )
        .arg("restart")
        .arg(
            crate::env::SERVICE_NAME
                .as_str(),
        )
        .output()
        .expect("failed to restart");

    let output =
        String::from_utf8_lossy(
            &cmd.stdout,
        );

    ctx.say(format!(
        "Restarted with output: {}",
        output
    ))
    .await?;

    Ok(())
}
