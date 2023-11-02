use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::InteractionResponseType;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;

use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use sqlx::PgPool;

use tracing::{info, error};

use crate::translator;

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
struct Guild {
    id: i64,
    guildname: String,
    language: String
}

pub async fn run(options: &[CommandDataOption], ctx: &Context, command: &ApplicationCommandInteraction, database: &PgPool) {
    let sub_option = options
        .get(0)
        .expect("Exoected op");
    // println!("{:#?}", sub_option);

    let guild_id = command.guild_id.unwrap().0 as i64;
    let guildname = ctx.http.get_guild(command.guild_id.unwrap().0).await.unwrap().name;

    let guild = sqlx::query_as::<_, Guild>("SELECT id, guildname, language FROM guilds WHERE id = $1")
            .bind(guild_id)
            .fetch_one(&*database)
            .await
            .map_err(|err| format!("Error in 'INSERT' Guild table: {}", err))
            .unwrap_or_else(|_| Guild {id: guild_id, guildname: guildname, language: "en".to_string() } );

    let _ = match sub_option.name.as_str() {
        "create" => create(&sub_option, &ctx, &command, &guild).await,
        _ => unknow(&ctx, &command, &guild).await
    };
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("selfrole").description("Manage roles for reaction")
        .name_localized("ru", "автороль")
        .description_localized("ru", "Управляйте ролями за реакцию!")
        .create_option(|option| {
            option
                .name("create")
                .name_localized("ru", "создать")
                .description("Creates a reaction under the message")
                .description_localized("ru", "Создаёт реакцию под сообщением")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|s_op| {
                    s_op
                        .name("channel")
                        .name_localized("ru", "канал")
                        .description("In which channel is the message to attach the reaction to?")
                        .description_localized("ru", "В каком канале сообщение, к которому надо прикрепить реакцию?")
                        .kind(CommandOptionType::Channel)
                        .required(true)
                })
                .create_sub_option(|s_op| {
                    s_op
                        .name("message_id")
                        .name_localized("ru", "id_сообщения")
                        .description("Specify the ID of the message to attach the reaction to")
                        .description_localized("ru", "Укажите ID сообщения, к которому надо прикрепить реакцию")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|s_op| {
                    s_op
                        .name("reaction")
                        .name_localized("ru", "реакция")
                        .description("Specify the ID of the message to attach the reaction to")
                        .description_localized("ru", "Укажите реакцию, которую надо прикрепить к сообщению")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|s_op| {
                    s_op
                        .name("role")
                        .name_localized("ru", "роль")
                        .description("Specify the ID of the message to attach the reaction to")
                        .description_localized("ru", "Укажите роль, которая будет даваться")
                        .kind(CommandOptionType::Role)
                        .required(true)
                })
                
        })

        .create_option(|option| {
            option
                .name("update")
                .name_localized("ru", "обновить")
                .description("Creates a reaction under the message")
                .description_localized("ru", "Создаёт реакцию под сообщением")
                .kind(CommandOptionType::SubCommand)
                .create_sub_option(|s_op| {
                    s_op
                        .name("channel")
                        .name_localized("ru", "канал")
                        .description("In which channel is the message to attach the reaction to?")
                        .description_localized("ru", "В каком канале сообщение, к которому надо прикрепить реакцию?")
                        .kind(CommandOptionType::Channel)
                        .required(true)
                })
                .create_sub_option(|s_op| {
                    s_op
                        .name("message")
                        .name_localized("ru", "сообщение")
                        .description("Specify the ID of the message to attach the reaction to")
                        .description_localized("ru", "Укажите ID сообщения, к которому надо прикрепить реакцию")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|s_op| {
                    s_op
                        .name("reaction")
                        .name_localized("ru", "реакция")
                        .description("Specify the ID of the message to attach the reaction to")
                        .description_localized("ru", "Укажите реакцию, которую надо прикрепить к сообщению")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
                .create_sub_option(|s_op| {
                    s_op
                        .name("role")
                        .name_localized("ru", "роль")
                        .description("Specify the ID of the message to attach the reaction to")
                        .description_localized("ru", "Укажите роль, которая будет даваться")
                        .kind(CommandOptionType::Role)
                        .required(true)
                })
        })
}

// ----------------------------------------------------------------------------- //

async fn create(sub_command: &CommandDataOption, ctx: &Context, command: &ApplicationCommandInteraction, guild: &Guild) {
    info!("Создалось!");
    let options = &sub_command.options;

    // println!("{:#?}", options);

    let channel_ar = options[0]
        .resolved
        .as_ref()
        .expect("Expected channel object");

    println!("{:#?}", channel_ar);

    // if let Op

    let text = translator::get(&guild.language, "selfrole-create", &fluent_bundle::FluentArgs::new());

    let interaction_response =
        command.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message|
                    message.content(text))
        });

    if let Err(why) = interaction_response.await {
        error!("Cannot respond to slash command: {}", why);
    }
}

async fn unknow(ctx: &Context, command: &ApplicationCommandInteraction, guild: &Guild) {
    let mut args = fluent_bundle::FluentArgs::new();
    args.set("command", command.data.name.clone());

    let text = translator::get(&guild.language, "selfrole-unknow", &args);

    let interaction_response =
        command.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message|
                    message.content(text))
        });

    if let Err(why) = interaction_response.await {
        error!("Cannot respond to slash command: {}", why);
    }
}