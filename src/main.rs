use std::time::Duration;

use rand::Rng;
use serenity::async_trait;
use serenity::client::ClientBuilder;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::model::prelude::command::Command;
use serenity::model::prelude::{GuildId, RoleId};
use serenity::prelude::{Context, EventHandler};
use tokio::{task, time};
use tracing::{error, info};

mod commands;

struct Handler {}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, event: Ready) {
        info!("Logged as {}", event.user.name);

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

        let _commands = Command::create_global_application_command(&ctx.http, |command| {
            commands::ping::register(command)
        })
        .await
        .expect("Could not register commands!");

        info!("Slash commands are registered");

        // Update member role colour every hour to something random
        task::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(3600));
            interval.tick().await; // First tick completes immediately

            loop {
                interval.tick().await;

                let result = GuildId::edit_role(guild_id, &ctx.http, &role_id, |role| {
                    role.colour(rand::thread_rng().gen_range(0..16777215))
                })
                .await;

                match result {
                    Ok(_) => {
                        info!("Successfully changed the colour of hour")
                    }
                    Err(err) => {
                        error!("Could not change colour of hour: {}", err)
                    }
                }
            }
        });
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!(
                "Received command interaction {} by {}",
                command.data.name, command.user
            );

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                _ => String::from("Böyle bir özelliğim yok :("),
            };

            let result = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await;

            match result {
                Ok(_) => {
                    info!(
                        "Successfully finished command interaction {} by {}",
                        command.data.name, command.user
                    )
                }
                Err(_) => {
                    error!(
                        "Error occurred before finishing command interaction {} by {}",
                        command.data.name, command.user
                    )
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let token = env!("DISCORD_TOKEN", "Expected DISCORD_TOKEN in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let mut client = ClientBuilder::new(token, intents)
        .event_handler(Handler {})
        .await
        .expect("Error logging in");

    client.start().await.expect("Error starting client");
}
