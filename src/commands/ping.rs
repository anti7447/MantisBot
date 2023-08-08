use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;

pub async fn run(_options: &[CommandDataOption], ctx: &Context, command: &ApplicationCommandInteraction) {

    let interaction_response =
        command.create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message|
                    message.content(format!(":ping_pong: pong!")))
        });

    if let Err(why) = interaction_response.await {
        eprintln!("Cannot respond to slash command: {}", why);
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("пинг").description("Команда пинга")
}