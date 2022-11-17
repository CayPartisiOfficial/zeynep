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

pub async fn start(http: Arc<Http>, guild_id: GuildId, role_id: RoleId) {
    let mut last_change = DayOfMonth::from_file_or_now(".cotd").await;

    tokio::task::spawn(async move {
        info!("Colour of the Day is started");
        let mut interval = tokio::time::interval(Duration::from_secs(600));
        loop {
            interval.tick().await;

            let now = DayOfMonth::now().await;
            // Continue if it is the same day
            if now == last_change {
                continue;
            }

            if guild_id
                .edit_role(&http, role_id, |role| role.colour(generate_random_colour()))
                .await
                .context("Cannot edit role")
                .trace_err()
                .is_ok()
            {
                last_change = now;
                last_change.save_to_file(".cotd").await.trace_err().ok();
                info!("Changed colour of the day");
            }
        }
    });
}

fn generate_random_colour() -> u64 {
    let colour = Colour(rand::thread_rng().gen_range(0..16777216));
    let lum = 0.299 * colour.r() as f32 + 0.587 * colour.g() as f32 + 0.114 * colour.b() as f32;

    if lum > 150.0 {
        colour.0 as u64
    } else {
        generate_random_colour()
    }
}

/// Represents day of month, if date was to be 2022-11-17, it would hold 17
struct DayOfMonth(pub u32);

impl DayOfMonth {
    async fn now() -> DayOfMonth {
        DayOfMonth(
            chrono_tz::Turkey
                .from_utc_datetime(&Utc::now().naive_utc())
                .day(),
        )
    }

    async fn from_file_or_now(file: &str) -> DayOfMonth {
        if let Ok(day_of_month) = fs::read(file).await {
            if let Ok(raw_bytes) = <[u8; 4]>::try_from(&day_of_month as &[u8]) {
                return DayOfMonth(u32::from_le_bytes(raw_bytes));
            }
        }

        Self::now().await
    }

    async fn save_to_file(&self, file: &str) -> Result<()> {
        fs::write(file, self.0.to_le_bytes())
            .await
            .context("Cannot write day of month to .cotd file")
    }
}

impl PartialEq<Self> for DayOfMonth {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
