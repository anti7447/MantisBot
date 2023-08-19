use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use sqlx::PgPool;

// use serde_json::json;
use tracing::info;

use crate::translator;

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
struct Guild {
    id: i64,
    guildname: String,
    language: String
}

pub async fn run(_options: &[CommandDataOption], ctx: &Context, command: &ApplicationCommandInteraction, database: &PgPool) {
    let guild_id = command.guild_id.unwrap().0 as i64;
    let guildname = ctx.http.get_guild(command.guild_id.unwrap().0).await.unwrap().name;

    let guild = sqlx::query_as::<_, Guild>("SELECT id, guildname, language FROM guilds WHERE id = $1")
            .bind(guild_id)
            .fetch_one(&*database)
            .await
            .map_err(|err| format!("Error in 'INSERT' Guild table: {}", err))
            .unwrap_or_else(|_| Guild {id: guild_id, guildname: guildname, language: "en".to_string() } );

    let text = translator::get(&guild.language, "ping-command", &fluent_bundle::FluentArgs::new());

    let interaction_response =
        command.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message|
                    message.content(text))
        });

    if let Err(why) = interaction_response.await {
        info!("Cannot respond to slash command: {}", why);
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("ping's command")
        .name_localized("ru", "пинг")
        .description_localized("ru", "Команда пинга")
}