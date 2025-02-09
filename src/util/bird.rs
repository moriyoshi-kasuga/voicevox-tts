use crate::cache::TtsChannel;
use crate::AnyResult;
use anyhow::anyhow;

use anyhow::Context as _;
use poise::serenity_prelude::Context;
use songbird::id::ChannelId;
use songbird::{id::GuildId, input::Input, Call, Songbird};
use std::sync::Arc;
use tokio::sync::Mutex;

#[inline]
pub async fn get_songbird(ctx: &Context) -> AnyResult<Arc<Songbird>> {
    let songbird = songbird::get(ctx)
        .await
        .context("Songbird voice client is not initialized")?;

    Ok(songbird)
}

pub async fn bird_join(
    manager: Arc<Songbird>,
    tts_channel: TtsChannel,
    guild_id: impl Into<GuildId>,
    channel_id: impl Into<ChannelId>,
    text_channel_id: impl Into<ChannelId>,
) -> AnyResult<()> {
    let guild_id = guild_id.into();
    let channel_id = channel_id.into();

    manager.join(guild_id, channel_id).await?;
    tts_channel.set(guild_id, text_channel_id.into()).await;

    Ok(())
}

pub async fn bird_laeve(
    manager: Arc<Songbird>,
    tts_channel: TtsChannel,
    guild_id: impl Into<GuildId>,
) -> AnyResult<()> {
    let guild_id = guild_id.into();

    manager.remove(guild_id).await?;
    tts_channel.remove(guild_id).await;

    Ok(())
}

pub async fn bird_enqueue<T: Into<Input>>(
    manager: Arc<Songbird>,
    guild_id: impl Into<GuildId>,
    audio: T,
) -> AnyResult<()> {
    let guild_id = guild_id.into();

    let call = get_call(manager, guild_id).await?;

    let mut handler = call.lock().await;
    handler.enqueue_input(audio.into()).await;

    Ok(())
}

async fn get_call(
    manager: Arc<Songbird>,
    guild_id: impl Into<GuildId>,
) -> AnyResult<Arc<Mutex<Call>>> {
    let guild_id = guild_id.into();

    let call = manager
        .get(guild_id)
        .ok_or_else(|| anyhow!("Failed to retrieve call for guild {}", guild_id))?;

    Ok(call)
}
