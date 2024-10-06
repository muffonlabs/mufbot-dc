use poise::serenity_prelude;

// check if there is permission or not
// returns true if user has role (which is what determines the permissions)
// returns false if user doesn't have the role (unauthorized)
pub async fn check_perms(
    ctx: crate::discord::commands::Context<'_>,
    user: &poise::serenity_prelude::User
) -> bool {
    let guild: serenity_prelude::PartialGuild =
        crate::discord::utils::get_guild(ctx).await;

    let role: &serenity_prelude::Role =
        crate::discord::utils::get_role(
            &guild
        );

    if !user
        .has_role(
            ctx.http(),
            &guild,
            role
        )
        .await
        .unwrap()
    {
        return false;
    }

    true
}

// Send no permission message
pub async fn send_no_perm(
    ctx: crate::discord::commands::Context<'_>
) -> Result<
    (),
    crate::discord::commands::Error
> {
    ctx
        .send(
            poise::CreateReply::default()
            .embed(
                poise::serenity_prelude::CreateEmbed::new()
                .title("Permission denied")
                .description(
                    "You are not allowed to perform this action."
                )
                .color(0xFF0000)
            )
            .ephemeral(true)
        ).await?;

    Ok(())
}

pub async fn check_and_send_no_perm(
    ctx: crate::discord::commands::Context<'_>,
    user: &poise::serenity_prelude::User
) -> Result<
    bool,
    crate::discord::commands::Error
> {
    if !check_perms(ctx, user).await {
        send_no_perm(ctx).await?;

        return Ok(false);
    }

    Ok(true)
}
