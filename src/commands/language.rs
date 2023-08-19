use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::command::CommandOptionType;
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

pub async fn run(options: &[CommandDataOption], ctx: &Context, command: &ApplicationCommandInteraction, database: &PgPool) {
    let language = options
        .get(0)
        .expect("Expected string option")
        .resolved
        .as_ref()
        .expect("Expected string object");

    let guild_id = command.guild_id.unwrap().0 as i64;
    let guildname = ctx.http.get_guild(command.guild_id.unwrap().0).await.unwrap().name;

    let mut guild: Guild = Guild {id: guild_id, guildname: guildname, language: "en".to_string() };
    let mut args = fluent_bundle::FluentArgs::new();

    if let CommandDataOptionValue::String(lang) = language {
        guild = sqlx::query_as::<_, Guild>("UPDATE guilds SET language=$1 WHERE id = 1137364407828086895 RETURNING id, guildname, language;")
            .bind(lang)
            .fetch_one(&*database)
            .await
            .map_err(|err| format!("Error in 'INSERT' Guild table: {}", err))
            .unwrap_or(guild);

        args.set("lang", lang.as_str());
    }

    

    let dst = translator::get(&guild.language, "lang-command-dst", &args);
    let ttl = translator::get(&guild.language, "lang-command-ttl", &args);

    let interaction_response =
        command.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message|
                    message.embed(|embed| {
                        embed
                            .title(ttl)
                            .description(dst)
                            .color(0x44ea62)
                    }))
        });

    if let Err(why) = interaction_response.await {
        info!("Cannot respond to slash command: {}", why);
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("language").description("changes the language")
        .name_localized("ru", "язык")
        .description_localized("ru", "Сменяет язык бота")

        .create_option(|option| {
            option
                .name("language")
                .name_localized("ru", "язык")
                .description("Select the language the bot into (this does not affect the names and descriptions of slash commands)")
                .description_localized("ru", "Выберите язык бота (это не влияет на имена и описания слэш-команд)")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice("English", "en")
                // .add_string_choice_localized("Англ", "en", ["ru"])
                .add_string_choice("Russian", "ru")
        })
}