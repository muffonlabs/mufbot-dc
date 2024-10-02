use poise::serenity_prelude::{
    self, Timestamp
};

#[poise::command(
    slash_command,
    prefix_command
)]

pub async fn reboot_server(
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

    let embed_author = serenity_prelude::CreateEmbedAuthor::new(&ctx.author().name)
        .icon_url(ctx.author().avatar_url().unwrap_or_default());

    let embed = serenity_prelude::CreateEmbed::default()
        .title("Reboot")
        .description("rebooting the server...")
        .color(0x804fb3)
        .author(embed_author)
        .timestamp(Timestamp::now());

    let reply =
        poise::CreateReply::default()
            .embed(embed);

    ctx.send(reply).await?;

    let _ =
        std::process::Command::new(
            "reboot"
        )
        .output()
        .expect("failed to reboot");

    Ok(())
}
