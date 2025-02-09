use anyhow::anyhow;
use std::sync::Arc;

use vvcore::VoicevoxCore;

use crate::AnyResult;

pub fn gen_tts(vvc: Arc<VoicevoxCore>, text: &str, speaker_id: u32) -> AnyResult<Vec<u8>> {
    let voice = vvc
        .tts_simple(text, speaker_id)
        .map_err(|err| anyhow!("failed to play TTS: {err:?}"))?;
    Ok(voice.as_slice().to_vec())
}
