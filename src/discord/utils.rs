pub mod no_perm;

use poise::serenity_prelude;

pub async fn get_guild(
    ctx: crate::discord::commands::Context<'_>
) -> serenity_prelude::PartialGuild {

    ctx.http()
        .get_guild(
            crate::env::GUILD_ID
                .as_str()
                .parse()
                .unwrap()
        )
        .await
        .unwrap()
}

fn get_role(
    guild: &serenity_prelude::PartialGuild
) -> &serenity_prelude::Role {

    guild
        .roles
        .get(
            &crate::env::ROLE_ID
                .as_str()
                .parse()
                .unwrap()
        )
        .unwrap()
}
