use poise::{
    serenity_prelude, CreateReply,
};

#[poise::command(
    slash_command,
    prefix_command
)]

pub async fn version(
    ctx: crate::discord::commands::Context<'_>,
) -> Result<
    (),
    crate::discord::commands::Error,
> {

    let version =
        env!("CARGO_PKG_VERSION");

    let embed = serenity_prelude::CreateEmbed::default()
        .title("Version")
        .description(format!("Current version: {}", version))
        .color(0x804fb3);

    let message =
        CreateReply::default()
            .embed(embed);

    ctx.send(message).await?;

    Ok(())
}
