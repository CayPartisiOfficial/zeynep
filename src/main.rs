use anyhow::{Context, Result};
use serenity::client::ClientBuilder;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::model::prelude::{GuildId, RoleId};
use serenity::prelude::EventHandler;
use serenity::{async_trait, prelude};
use tracing::{error, info};

use crate::cotd::Cotd;

mod commands;
mod cotd;

trait ResultTraceErr {
    fn trace_err(self) -> Self;
}

impl<T> ResultTraceErr for Result<T> {
    /// If result is an error, traces it and returns self
    fn trace_err(self) -> Self {
        if let Err(err) = &self {
            error!("{}", err);
        }
        self
    }
}

struct Handler {
    guild_id: GuildId,
    role_id: RoleId,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: prelude::Context, event: Ready) {
        info!("Logged as {}", event.user.name);

        commands::register_slash_commands(ctx.http.clone()).await;
        Cotd::start(ctx.http, self.guild_id, self.role_id).await;
    }

    async fn interaction_create(&self, ctx: prelude::Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!(
                "Received command interaction {} by {}",
                command.data.name, command.user
            );

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                _ => String::from("Böyle bir özelliğim yok :("),
            };

            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
                .context(format!(
                    "Error occurred before finishing command interaction {} by {}",
                    command.data.name, command.user
                ))
                .trace_err()
                .ok();
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = env!("DISCORD_TOKEN", "Expected DISCORD_TOKEN in the environment");
    let guild_id = GuildId(
        env!("GUILD_ID", "Expected GUILD_ID in environment")
            .parse()
            .expect("GUILD_ID must be a number"),
    );

    let role_id = RoleId(
        env!("ROLE_ID", "Expected ROLE_ID in environment")
            .parse()
            .expect("ROLE_ID must be a number"),
    );

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = ClientBuilder::new(token, intents)
        .event_handler(Handler { guild_id, role_id })
        .await
        .expect("Error logging in");
    client.start().await.expect("Error starting client");
}
