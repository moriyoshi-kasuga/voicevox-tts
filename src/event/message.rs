use poise::serenity_prelude::{Context, Message};

use crate::{
    util::{
        bird::{bird_enqueue, get_songbird},
        get_tts_channel, get_voice_config, get_vvc,
        vvc::gen_tts,
    },
    AnyResult,
};

pub async fn handle_message(ctx: &Context, message: &Message) -> AnyResult<()> {
    let Some(guild_id) = message.guild_id else {
        return Ok(());
    };

    let manager = get_songbird(ctx).await?;
    let Some(call) = manager.get(guild_id) else {
        return Ok(());
    };

    let Some(_) = call.lock().await.current_channel() else {
        return Ok(());
    };

    let tts_channel = get_tts_channel(ctx).await?;

    if !tts_channel
        .has_eq(guild_id.into(), message.channel_id.into())
        .await
    {
        return Ok(());
    }

    let voice_config = get_voice_config(ctx).await?;
    let vvc = get_vvc(ctx).await?;

    let audio = gen_tts(vvc, &message.content, voice_config.default_speaker_id)?;

    bird_enqueue(manager, guild_id, audio).await?;

    Ok(())
}
