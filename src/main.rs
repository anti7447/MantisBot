use shuttle_runtime::CustomError;
use sqlx::{PgPool, Executor};
use tracing::info;
use axum::Router;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use axum::routing::get;

mod commands;

mod router;
use router::*;

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
    #[shuttle_aws_rds::Postgres(
        local_uri = "postgres://mantis:{secrets.PASSWORD}@localhost/mantisdb"
    )] pool: PgPool,
) -> Result<Service, shuttle_runtime::Error> {
    pool.execute(include_str!("../sql/start.sql"))
        .await
        .map_err(CustomError::new)?;

    let token = secrets.get("DISCORD_TOKEN").unwrap_or_else(|| {
        info!("`DISCORD_TOKEN` not found!");
        "Token not found".to_string()
    });

    let bot = Bot { database: pool.clone() };
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let client = Client::builder(&token, intents)
        .event_handler(bot)
        
        .await
        .expect("Err creating client");

    let router = Router::new()
        .route("/", get(hello_world))
        .with_state(pool);

    Ok(Service {
        discord_bot: client,
        router: router
    })
}
