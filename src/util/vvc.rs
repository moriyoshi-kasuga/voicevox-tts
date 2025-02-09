use anyhow::anyhow;
use poise::serenity_prelude::GuildId;
use std::sync::Arc;

use vvcore::VoicevoxCore;

use crate::{cache::VoiceCache, config::dictionary::Dictionary, AnyResult};

pub async fn gen_tts(
    text: &str,
    vvc: Arc<VoicevoxCore>,
    cache: VoiceCache,
    dict: Dictionary,
    guild_id: GuildId,
    speaker_id: u32,
) -> AnyResult<Vec<u8>> {
    let text = dict.replace(guild_id, text).await;
    let text = text.as_str();

    let boxed = text.into();
    let voice = cache
        .try_get_with((boxed, speaker_id), async move {
            gen_tts_without_cache(vvc, text, speaker_id)
                .inspect(|_| {
                    tracing::info!("success generate voice of {text} with speaker {speaker_id}")
                })
                .inspect_err(|_| {
                    tracing::error!("failed generate voice of {text} with speaker {speaker_id}")
                })
        })
        .await
        .map_err(|v| {
            let error = Arc::into_inner(v);
            match error {
                Some(err) => err,
                None => "failed to generate voice".into(),
            }
        })?;
    Ok(voice)
}

pub fn gen_tts_without_cache(
    vvc: Arc<VoicevoxCore>,
    text: &str,
    speaker_id: u32,
) -> AnyResult<Vec<u8>> {
    let voice = vvc
        .tts_simple(text, speaker_id)
        .map_err(|err| anyhow!("failed to play TTS: {err:?}"))?;
    Ok(voice.as_slice().to_vec())
}
