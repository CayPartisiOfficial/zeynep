use std::sync::Arc;

use anyhow::{Context, Result};
use chrono::{Datelike, TimeZone, Utc};
use rand::Rng;
use serenity::http::Http;
use serenity::model::id::RoleId;
use serenity::model::prelude::GuildId;
use serenity::utils::Colour;
use tokio::fs;
use tokio::time::Duration;
use tracing::info;

use crate::ResultTraceErr;

/// COTD stands for Colour of the Day, this struct implements changing colour of member role every day
pub struct Cotd {
    colour: Colour,
}

impl Cotd {
    pub async fn start(http: Arc<Http>, guild_id: GuildId, role_id: RoleId) {
        let mut cotd = Cotd {
            colour: generate_random_colour(),
        };

        tokio::task::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(600));

            loop {
                interval.tick().await;

                if cotd.should_update().await.trace_err().unwrap_or(false) {
                    cotd.update(&http, guild_id, role_id).await.trace_err().ok();
                }
            }
        });

        info!("Colour of the Day is started");
    }

    async fn update(&mut self, http: &Arc<Http>, guild_id: GuildId, role_id: RoleId) -> Result<()> {
        self.update_colour().await?;
        guild_id
            .edit_role(http, role_id, |role| role.colour(self.colour.0 as u64))
            .await
            .context("Cannot edit role")?;
        info!("Updated colour of the day to 0x{}", self.colour.hex());
        Ok(())
    }

    async fn update_colour(&mut self) -> Result<()> {
        self.colour = generate_random_colour();
        self.save_timestamp().await?;
        Ok(())
    }

    async fn should_update(&self) -> Result<bool> {
        if let Ok(day_of_month) = fs::read(".cotd").await {
            let raw_bytes = <[u8; 4]>::try_from(&day_of_month as &[u8])
                .context("Invalid day of month in .cotd file")?;
            let day_of_month = u32::from_le_bytes(raw_bytes);

            Ok(day_of_month_today() - day_of_month >= 1)
        } else {
            self.save_timestamp().await?;

            Ok(true)
        }
    }

    async fn save_timestamp(&self) -> Result<()> {
        fs::write(".cotd", day_of_month_today().to_le_bytes())
            .await
            .context("Cannot write day of month to .cotd file")
    }
}

fn day_of_month_today() -> u32 {
    chrono_tz::Turkey
        .from_utc_datetime(&Utc::now().naive_utc())
        .day()
}

fn generate_random_colour() -> Colour {
    let colour = Colour(rand::thread_rng().gen_range(0..16777216));
    let lum = 0.299 * colour.r() as f32 + 0.587 * colour.g() as f32 + 0.114 * colour.b() as f32;
    if lum > 150.0 {
        colour
    } else {
        generate_random_colour()
    }
}
