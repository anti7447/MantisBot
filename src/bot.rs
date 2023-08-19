use serenity::model::prelude::{Interaction, InteractionResponseType};
use serenity::{async_trait, model::prelude::GuildId};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use tracing::info;
use sqlx::PgPool;

use crate::commands;

pub struct Bot {
    pub database: PgPool
}

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
struct Guild {
    id: i64,
    guildname: String,
    language: String
}

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i64,
    username: String
}

#[allow(dead_code)]
#[derive(Debug, sqlx::FromRow)]
struct Member {
    user_id: i64,
    guild_id: i64
}

// ---------------------- Event Handler ----------------------------- //
#[async_trait]
impl EventHandler for Bot {

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(1137364407828086895);

        let _ = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::language::register(command))
        }).await.unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let _ = match command.data.name.as_str() {
                "hello" => (),
                "ping" => commands::ping::run(&command.data.options, &ctx, &command, &self.database).await,
                "language" => commands::language::run(&command.data.options, &ctx, &command, &self.database).await,
                command_name => unknow_command_run(command_name, &ctx, &command).await,
            };
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let user_id = msg.author.id.0 as i64;
        let username = msg.author.name;

        let guild_id = msg.guild_id.unwrap().0 as i64;
        let guildname = ctx.http.get_guild(msg.guild_id.unwrap().0).await.unwrap().name;

        let _ = sqlx::query_as::<_, User>(include_str!("../sql/insert/insert_user.sql"))
            .bind(user_id)
            .bind(username)
            .fetch_one(&self.database)
            .await
            .map_err(|err| format!("Error in 'INSERT' User table: {}", err));

        let _ = sqlx::query_as::<_, Guild>(include_str!("../sql/insert/insert_guild.sql"))
            .bind(guild_id)
            .bind(guildname)
            .bind("en")
            .fetch_one(&self.database)
            .await
            .map_err(|err| format!("Error in 'INSERT' Guild table: {}", err));

        let _ = sqlx::query_as::<_, Member>(include_str!("../sql/insert/insert_member.sql"))
            .bind(user_id)
            .bind(guild_id)
            .fetch_one(&self.database)
            .await
            .map_err(|err| format!("Error in 'INSERT' Member table: {}", err));
    }
}
// --------------------------------------------------------------- //

async fn unknow_command_run(command_name: &str, ctx: &Context, command: &ApplicationCommandInteraction) {
    info!("{} comman not found", command_name);
    let interaction_response =
        command.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message|
                    message
                        .content(format!("{} comman not found", command_name))
                        .ephemeral(true))
        });

    if let Err(why) = interaction_response.await {
        eprintln!("Cannot respond to slash command: {}", why);
    }
}