use serenity::model::prelude::{Interaction, InteractionResponseType};
use serenity::{async_trait, model::prelude::GuildId};
use serenity::model::gateway::Ready;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::*;
use tracing::info;

use crate::commands;

pub struct Bot;

#[async_trait]
impl EventHandler for Bot {

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_id = GuildId(1137364407828086895);

        let _ = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
        }).await.unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let _ = match command.data.name.as_str() {
                "hello" => (),
                "ping" => commands::ping::run(&command.data.options, &ctx, &command).await,
                command_name => unknow_command_run(command_name, &ctx, &command).await,
            };
        }
    }
}

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