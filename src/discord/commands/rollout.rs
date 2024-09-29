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

    if !crate::discord::utils::no_perm::check_and_send_no_perm(
        ctx,
        ctx.author()
    ).await? {
        return Ok(());
    };

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

    while let Some(component_interaction) = serenity_prelude::ComponentInteractionCollector::new(ctx.serenity_context())
        .timeout(std::time::Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id.starts_with("approve-") || mci.data.custom_id.starts_with("reject-"))
        .await
    {

        // check permissions

        crate::discord::utils::no_perm::check_and_send_no_perm(ctx, &component_interaction.user).await?;

        // If interactor and creator are same, disregard it because
        // author can't approve or deny their own rollout
        if component_interaction.user.id == ctx.author().id {
            ctx
            .send(
                poise::CreateReply::default()
                .embed(
                    poise::serenity_prelude::CreateEmbed::new()
                    .title("Invalid action")
                    .description(
                        "You are not allowed to approve or deny your own rollout"
                    )
                    .color(0xFF0000)
                )
                .ephemeral(true)
            ).await?;

            component_interaction
                .create_response(
                    ctx,
                    serenity_prelude::CreateInteractionResponse::Acknowledge
                ).await?;

            return Ok(());
        }

        if component_interaction.data.custom_id == format!("approve-{}", version) {
            let build_queue = ctx
                .data()
                .build_queue
                .lock()
                .await;

            let success = build_queue
                .approve_rollout(&version, component_interaction.user.id.get())
                .expect(
                    "failed to approve rollout",
                );

            let rollout_object = build_queue
                .get_rollout(&version)
                .expect("failed to get rollout");

            drop(build_queue);

            if !success {
                let embed = serenity_prelude::CreateEmbed::default()
                    .title("Invalid action")
                    .description("There was an error approving the rollout. This version may have already been approved.")
                    .color(0xFF0000);

                let reply = poise::CreateReply::default()
                    .embed(embed);

                ctx.send(reply).await?;
                component_interaction.create_response(ctx, serenity_prelude::CreateInteractionResponse::Acknowledge).await?;
                continue;
            }

            let approval = rollout_object.status == "approved";

            let extra = if approval {
                let _ = crate::github::start_rollout(&version).await;
                ". The rollout has been started."
            } else {
                ". The rollout has not been started because it has not been approved by enough people."
            };

            let embed_author = serenity_prelude::CreateEmbedAuthor::new(&component_interaction.user.name)
                .icon_url(component_interaction.user.avatar_url().unwrap_or_default())
                .url(component_interaction.user.avatar_url().unwrap_or_default());

            let embed = serenity_prelude::CreateEmbed::default()
                .title("Rollout Approved")
                .description(format!("Rollout of version {} has been approved by {}{}. ID: {}", version, &component_interaction.user.name, extra, &component_interaction.data.custom_id))
                .author(embed_author)
                .color(0x00FF00);

            let reply = poise::CreateReply::default()
                .embed(embed);

            ctx.send(reply).await?;
            component_interaction.create_response(ctx, serenity_prelude::CreateInteractionResponse::Acknowledge).await?;
        } else if component_interaction.data.custom_id == format!("reject-{}", version) {
            let build_queue = ctx
                .data()
                .build_queue
                .lock()
                .await;

            let success = build_queue
                .reject_rollout(&version, component_interaction.user.id.get())
                .expect(
                    "failed to reject rollout",
                );

            drop(build_queue);

            if !success {
                let embed = serenity_prelude::CreateEmbed::default()
                    .title("Invalid action")
                    .description("There was an error rejecting the rollout. This version may have already been rejected.")
                    .color(0xFF0000);

                let reply = poise::CreateReply::default()
                    .embed(embed);

                ctx.send(reply).await?;
                component_interaction.create_response(ctx, serenity_prelude::CreateInteractionResponse::Acknowledge).await?;
                continue;
            }

            let embed_author = serenity_prelude::CreateEmbedAuthor::new(&component_interaction.user.name)
                .icon_url(component_interaction.user.avatar_url().unwrap_or_default())
                .url(component_interaction.user.avatar_url().unwrap_or_default());

            let embed = serenity_prelude::CreateEmbed::default()
                .title("Rollout Rejected")
                .description(format!("Rollout of version {} has been rejected by {}. Cancelling rollout", version, &component_interaction.user.name))
                .author(embed_author)
                .color(0xFF0000);

            let reply = poise::CreateReply::default()
                .embed(embed);

            ctx.send(reply).await?;
            component_interaction.create_response(ctx, serenity_prelude::CreateInteractionResponse::Acknowledge).await?;
        }
    }

    Ok(())
}
