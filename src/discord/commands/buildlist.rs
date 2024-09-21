#[poise::command(slash_command, prefix_command)]

pub async fn buildlist(
    ctx: crate::discord::commands::Context<'_>,
) -> Result<(), crate::discord::commands::Error> {

    // Auth check
    let guild_id = std::env::var("MUFFON_GUILD_ID").expect("missing MUFFON_GUILD_ID");

    let role_id = std::env::var("ROLLOUT_GROUP_ID").expect("missing ROLLOUT_GROUP_ID");

    let guild = ctx
        .http()
        .get_guild(guild_id.parse().unwrap())
        .await?;

    let role = guild
        .roles
        .get(&role_id.parse().unwrap())
        .unwrap();

    if !ctx
        .author()
        .has_role(ctx.http(), &guild, role)
        .await?
    {

        ctx.say("You don't have permission to use this command")
            .await?;

        return Ok(());
    }

    let mut response = String::from("```");

    response.push_str("Version Status Approvals Rejections Created At\n");

    let build_queue = ctx
        .data()
        .build_queue
        .lock()
        .await;

    for build in build_queue.get_builds() {

        response.push_str(&build);

        response.push('\n');
    }

    drop(build_queue);

    response.push_str("```");

    ctx.say(response).await?;

    Ok(())
}
