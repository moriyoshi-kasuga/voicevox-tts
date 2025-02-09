use std::sync::Arc;

use anyhow::Context as _;
use poise::serenity_prelude::Context;
use songbird::typemap::TypeMapKey;
use vvcore::VoicevoxCore;

use crate::{
    cache::{TtsChannel, TtsChannelKey, VoiceCache, VoiceCacheKey},
    config::{dictionary::Dictionary, messages::VoiceConfig},
    AnyResult,
};

pub mod bird;
pub mod discord;
pub mod vvc;

pub async fn get_vvc(ctx: &Context) -> AnyResult<Arc<VoicevoxCore>> {
    let data = ctx.data.read().await;

    let vvc = data
        .get::<VoicevoxCoreKey>()
        .cloned()
        .context("VoicevoxCore is not initialized")?;

    Ok(vvc)
}

pub struct VoicevoxCoreKey;

impl TypeMapKey for VoicevoxCoreKey {
    type Value = Arc<VoicevoxCore>;
}

pub async fn get_voice_config(ctx: &Context) -> AnyResult<Arc<VoiceConfig>> {
    let data = ctx.data.read().await;

    let vvc = data
        .get::<VoiceConfigKey>()
        .cloned()
        .context("VoiceConfig is not initialized")?;

    Ok(vvc)
}

pub struct VoiceConfigKey;

impl TypeMapKey for VoiceConfigKey {
    type Value = Arc<VoiceConfig>;
}

pub async fn get_dict(ctx: &Context) -> AnyResult<Dictionary> {
    let data = ctx.data.read().await;

    let vvc = data
        .get::<DictionaryKey>()
        .cloned()
        .context("Dictionary is not initialized")?;

    Ok(vvc)
}

pub struct DictionaryKey;

impl TypeMapKey for DictionaryKey {
    type Value = Dictionary;
}

pub async fn get_tts_channel(ctx: &Context) -> AnyResult<TtsChannel> {
    let data = ctx.data.read().await;

    let tts_channel = data
        .get::<TtsChannelKey>()
        .cloned()
        .context("TtsChannel is not initialized")?;

    Ok(tts_channel)
}

pub async fn get_voice_cache(ctx: &Context) -> AnyResult<VoiceCache> {
    let data = ctx.data.read().await;

    let voice_cache = data
        .get::<VoiceCacheKey>()
        .cloned()
        .context("VoiceCache is not initialized")?;

    Ok(voice_cache)
}
