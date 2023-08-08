use axum::Router;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;

mod commands;

mod router;
use router::build_router;

mod bot;
use bot::Bot;

pub struct Service {
    discord_bot: Client,
    router: Router,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for Service {
    async fn bind(
        mut self,
        addr: std::net::SocketAddr,
    ) -> Result<(), shuttle_runtime::Error> {
        
        let router = self.router;

        let serve_router = axum::Server::bind(&addr).serve(router.into_make_service());

        tokio::select!(
            _ = self.discord_bot.start() => {},
            _ = serve_router => {}
        );

        Ok(())
    }
}

#[shuttle_runtime::main]
async fn init(
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> Result<Service, shuttle_runtime::Error> {
    let token = secrets.get("DISCORD_TOKEN").unwrap_or_else(|| {
        println!("`DISCORD_TOKEN` not found!");
        "Token not found".to_string()
    });
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    let router = build_router();

    Ok(Service {
        discord_bot: client,
        router: router
    })
}
