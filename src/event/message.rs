use poise::serenity_prelude::{Context, Message};

use crate::{
    get_bot_data,
    util::{
        bird::{bird_enqueue, get_songbird},
        discord::is_human,
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

    let bot_data = get_bot_data(ctx).await;
    let tts_channel = &bot_data.tts_channel;

    if !tts_channel
        .has_eq(guild_id.into(), message.channel_id.into())
        .await
    {
        return Ok(());
    }

    let is_human = is_human(&message.author);

    if !is_human {
        return Ok(());
    }

    let voice_config = &bot_data.config;
    let vvc = &bot_data.vvc;
    let cache = &bot_data.voice_cache;
    let dict = &bot_data.dict;

    let audio = gen_tts(
        &message.content,
        vvc.clone(),
        cache.clone(),
        dict.clone(),
        guild_id,
        voice_config.default_speaker_id,
    )
    .await?;

    bird_enqueue(manager, guild_id, audio).await?;

    Ok(())
}
