use poise::serenity_prelude;

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
        .queue_rollout(
            &version,
            ctx.author().id.get()
        )
        .expect(
            "failed to queue build"
        );

    drop(build_queue);

    let response = {

        let embed = serenity_prelude::CreateEmbed::default()
            .title("Rollout Queued")
            .description(format!("Rollout of version {} has been queued", version));

        let components = vec![
            serenity_prelude::CreateActionRow::Buttons(
                vec![
                    serenity_prelude::CreateButton::new("approve")
                        .style(serenity_prelude::ButtonStyle::Success)
                        .label("Approve")
                        .custom_id(format!("approve-{}", version)),
                    serenity_prelude::CreateButton::new("reject")
                        .style(serenity_prelude::ButtonStyle::Danger)
                        .label("Reject")
                        .custom_id(format!("reject-{}", version)),
                ]
        )];

        poise::CreateReply::default()
            .embed(embed)
            .components(components)
    };

    ctx.send(response).await?;

    while let Some(mci) = serenity_prelude::ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id.starts_with("approve-") || mci.data.custom_id.starts_with("reject-"))
        .await
    {

        // check permissions

        if !mci.
            user
            .has_role(
                ctx.http(),
                &guild,
                role
            )
            .await?
        {

            let embed = serenity_prelude::CreateEmbed::default()
                .title("Oops!")
                .description(format!("You don't have permission to use this interaction"))
                .author(
                    serenity_prelude::CreateEmbedAuthor::new(&mci.user.name)
                        .icon_url(mci.user.avatar_url().unwrap_or_default())
                        .url(mci.user.avatar_url().unwrap_or_default())
                )
                .color(0xFF0000);

            let reply = serenity_prelude::CreateInteractionResponseMessage::new()
                .embed(embed);

            mci.create_response(ctx, serenity_prelude::CreateInteractionResponse::Message(reply)).await?;

            continue;
        }

        if mci.data.custom_id == format!("approve-{}", version) {
            let build_queue = ctx
                .data()
                .build_queue
                .lock()
                .await;

            let success = build_queue
                .approve_rollout(&version, mci.user.id.get())
                .expect(
                    "failed to approve rollout",
                );

            let rollout_object = build_queue
                .get_rollout(&version)
                .expect("failed to get rollout");

            drop(build_queue);

            if !success {
                let embed = serenity_prelude::CreateEmbed::default()
                    .title("Oops!")
                    .description("There was an error approving the rollout. This version may have already been approved or rejected or it was your own rollout.")
                    .color(0xFF0000);

                let reply = poise::CreateReply::default()
                    .embed(embed);

                ctx.send(reply).await?;
                mci.create_response(ctx, serenity_prelude::CreateInteractionResponse::Acknowledge).await?;
                continue;
            }

            let approval = rollout_object.status == "approved";

            let extra = if approval {
                let _ = crate::github::start_rollout(&version).await;
                ". The rollout has been started."
            } else {
                ". The rollout has not been started because it has not been approved by enough people."
            };

            let embed_author = serenity_prelude::CreateEmbedAuthor::new(&mci.user.name)
                .icon_url(mci.user.avatar_url().unwrap_or_default())
                .url(mci.user.avatar_url().unwrap_or_default());

            let embed = serenity_prelude::CreateEmbed::default()
                .title("Rollout Approved")
                .description(format!("Rollout of version {} has been approved by {}{} FUCK THIS SHIT {}", version, &mci.user.name, extra, &mci.data.custom_id))
                .author(embed_author)
                .color(0x00FF00);

            let reply = poise::CreateReply::default()
                .embed(embed);

            ctx.send(reply).await?;
            mci.create_response(ctx, serenity_prelude::CreateInteractionResponse::Acknowledge).await?;
        } else if mci.data.custom_id == format!("reject-{}", version) {
            let build_queue = ctx
                .data()
                .build_queue
                .lock()
                .await;

            let success = build_queue
                .reject_rollout(&version, mci.user.id.get())
                .expect(
                    "failed to reject rollout",
                );

            drop(build_queue);

            if !success {
                let embed = serenity_prelude::CreateEmbed::default()
                    .title("Oops!")
                    .description("There was an error rejecting the rollout. This version may have already been approved or rejected or it was your own rollout.")
                    .color(0xFF0000);

                let reply = poise::CreateReply::default()
                    .embed(embed);

                ctx.send(reply).await?;
                mci.create_response(ctx, serenity_prelude::CreateInteractionResponse::Acknowledge).await?;
                continue;
            }

            let embed_author = serenity_prelude::CreateEmbedAuthor::new(&mci.user.name)
                .icon_url(mci.user.avatar_url().unwrap_or_default())
                .url(mci.user.avatar_url().unwrap_or_default());

            let embed = serenity_prelude::CreateEmbed::default()
                .title("Rollout Rejected")
                .description(format!("Rollout of version {} has been rejected by {}. Cancelling rollout", version, &mci.user.name))
                .author(embed_author)
                .color(0xFF0000);

            let reply = poise::CreateReply::default()
                .embed(embed);

            ctx.send(reply).await?;
            mci.create_response(ctx, serenity_prelude::CreateInteractionResponse::Acknowledge).await?;
        }
    }

    Ok(())
}
