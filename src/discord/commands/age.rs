use poise::serenity_prelude;

#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: crate::discord::commands::Context<'_>,
    #[description = "Selected user"] user: Option<serenity_prelude::User>,
) -> Result<(), crate::discord::commands::Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}
