use crate::AnyResult;

use anyhow::Context as _;
use poise::serenity_prelude::Context;
use songbird::{id::GuildId, Songbird};
use std::sync::Arc;

#[inline]
pub async fn get_songbird(ctx: &Context) -> AnyResult<Arc<Songbird>> {
    let songbird = songbird::get(ctx)
        .await
        .context("Songbird voice client is not initialized")?;

    Ok(songbird)
}

pub async fn bird_enqueue<T: AsRef<[u8]> + Send + Sync + 'static>(
    ctx: &Context,
    guild_id: impl Into<GuildId>,
    audio: T,
) -> AnyResult<()> {
    let manager = get_songbird(ctx).await?;

    let guild_id = guild_id.into();

    let call = manager
        .get(guild_id)
        .ok_or_else(|| anyhow::anyhow!("Failed to retrieve call for guild {}", guild_id))?;

    let mut handler = call.lock().await;
    handler.enqueue_input(audio.into()).await;

    Ok(())
}
