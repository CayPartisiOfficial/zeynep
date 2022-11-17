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
    last_change: DayOfMonth,
}

impl Cotd {
    pub async fn start(http: Arc<Http>, guild_id: GuildId, role_id: RoleId) {
        let mut cotd = Cotd {
            colour: generate_random_colour(),
            last_change: DayOfMonth::from_file_or_now(".cotd").await,
        };

        tokio::task::spawn(async move {
            info!("Colour of the Day is started");

            let mut interval = tokio::time::interval(Duration::from_secs(600));
            loop {
                interval.tick().await;

                let now = DayOfMonth::now().await;
                // Continue if it is the same day
                if now == cotd.last_change {
                    continue;
                }

                cotd.colour = generate_random_colour();

                if guild_id
                    .edit_role(&http, role_id, |role| role.colour(cotd.colour.0 as u64))
                    .await
                    .context("Cannot edit role")
                    .trace_err()
                    .is_ok()
                {
                    cotd.last_change = now;
                    cotd.last_change
                        .save_to_file(".cotd")
                        .await
                        .trace_err()
                        .ok();
                }

                info!("Updated colour of the day to 0x{}", cotd.colour.hex());
            }
        });
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

fn generate_random_colour() -> Colour {
    let colour = Colour(rand::thread_rng().gen_range(0..16777216));
    let lum = 0.299 * colour.r() as f32 + 0.587 * colour.g() as f32 + 0.114 * colour.b() as f32;

    if lum > 150.0 {
        colour
    } else {
        generate_random_colour()
    }
}
