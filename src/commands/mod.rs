use std::sync::Arc;

use anyhow::Context;
use serenity::http::Http;
use serenity::model::prelude::command::Command;
use tracing::info;

use crate::ResultTraceErr;

pub mod ping;

pub async fn register_slash_commands(http: Arc<Http>) {
    Command::create_global_application_command(http, |command| ping::register(command))
        .await
        .context("Could not register commands!")
        .trace_err()
        .ok();

    info!("Slash commands are registered");
}
