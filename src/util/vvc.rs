use anyhow::anyhow;
use poise::serenity_prelude::GuildId;
use std::sync::Arc;

use vvcore::VoicevoxCore;

use crate::AnyResult;

pub const VOICE_SELECT_MENU_CUSTOM_ID: &str = "voicevox_voice";

pub const VOICE_CHARACTER: &[(&str, u32)] = &comptime::all_voices!();

pub async fn gen_tts(
    text: &str,
    bot_data: Arc<crate::BotData>,
    guild_id: GuildId,
    speaker_id: u32,
) -> AnyResult<Vec<u8>> {
    let text = if text.chars().count() > bot_data.config.voice.max_message_length {
        text.chars()
            .take(bot_data.config.voice.max_message_length)
            .collect::<String>()
    } else {
        text.to_string()
    };
    let text = bot_data.dict.replace(guild_id, &text).await;
    let text = text.as_str();

    let vvc = Arc::clone(&bot_data.vvc);
    let boxed = text.into();
    let voice = bot_data
        .voice_cache
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
