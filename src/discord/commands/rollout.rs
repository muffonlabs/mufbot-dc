use poise::serenity_prelude;

use crate::github;

#[poise::command(
    slash_command,
    prefix_command
)]

pub async fn create_rollout(
    ctx: crate::discord::commands::Context<'_>,
    #[description = "Version to roll out"]
    version: String
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

    // Add rollout to queue
    let build_queue = ctx
        .data()
        .build_queue
        .lock()
        .await;

    build_queue
        .queue_rollout(&version)
        .expect(
            "failed to queue build"
        );

    drop(build_queue);

    // let response = {
    //     let embed = serenity_prelude::CreateEmbed::default()
    //         .title("Rollout Queued")
    //         .description(format!("Rollout of version {} has been queued", version));

    //     let components = vec![
    //         serenity_prelude::CreateActionRow::Buttons(
    //             vec![
    //                 serenity_prelude::CreateButton::new("approve")
    //                     .style(serenity_prelude::ButtonStyle::Primary)
    //                     .label("Approve")
    //                     .custom_id("approve"),
    //                 serenity_prelude::CreateButton::new("reject")
    //                     .style(serenity_prelude::ButtonStyle::Danger)
    //                     .label("Reject")
    //                     .custom_id("reject")]
    //     )];

    //     poise::CreateReply::default()
    //         .embed(embed)
    //         .components(components)
    // };

    github::start_rollout(&version)
        .await?;

    let response = {

        let embed = serenity_prelude::CreateEmbed::default()
            .title("Rollout Started")
            .description(format!("Rollout of version {} has been started", version))
            .color(0x00FF00);

        poise::CreateReply::default()
            .embed(embed)
    };

    ctx.send(response).await?;

    // while let Some(mci) = serenity_prelude::ComponentInteractionCollector::new(ctx.serenity_context())
    //     .timeout(std::time::Duration::from_secs(120))
    //     .filter(move |mci| mci.data.custom_id == "approve" || mci.data.custom_id == "reject")
    //     .await
    // {
    //     if &mci.data.custom_id == "approve" {
    //         // let mut build_queue = ctx
    //         //     .data()
    //         //     .build_queue
    //         //     .lock()
    //         //     .await;

    //         // build_queue
    //         //     .approve_rollout(&version)
    //         //     .expect(
    //         //         "failed to approve rollout",
    //         //     );

    //         // drop(build_queue);

    //         let embed_author = serenity_prelude::CreateEmbedAuthor::new(&mci.user.name)
    //             .icon_url(&mci.user.avatar_url().unwrap_or_default())
    //             .url(&mci.user.avatar_url().unwrap_or_default());

    //         let embed = serenity_prelude::CreateEmbed::default()
    //             .title("Rollout Approved")
    //             .description(format!("Rollout of version {} has been approved by {}", version, &mci.user.name))
    //             .author(embed_author)
    //             .color(0x00FF00);

    //         let reply = poise::CreateReply::default()
    //             .embed(embed);

    //         ctx.send(reply).await?;
    //     } else {
    //         // let mut build_queue = ctx
    //         //     .data()
    //         //     .build_queue
    //         //     .lock()
    //         //     .await;

    //         // build_queue
    //         //     .reject_rollout(&version)
    //         //     .expect(
    //         //         "failed to reject rollout",
    //         //     );

    //         // drop(build_queue);

    //         let embed_author = serenity_prelude::CreateEmbedAuthor::new(&mci.user.name)
    //             .icon_url(&mci.user.avatar_url().unwrap_or_default())
    //             .url(&mci.user.avatar_url().unwrap_or_default());

    //         let embed = serenity_prelude::CreateEmbed::default()
    //             .title("Rollout Rejected")
    //             .description(format!("Rollout of version {} has been rejected by {}. Cancelling rollout", version, &mci.user.name))
    //             .author(embed_author)
    //             .color(0xFF0000);

    //         let reply = poise::CreateReply::default()
    //             .embed(embed);

    //         ctx.send(reply).await?;
    //     }
    //     mci.create_response(ctx, serenity_prelude::CreateInteractionResponse::Acknowledge).await?;
    // }

    Ok(())
}
