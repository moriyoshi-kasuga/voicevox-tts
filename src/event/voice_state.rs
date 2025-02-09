use std::sync::Arc;

use poise::serenity_prelude::{Channel, Context, GuildId, Member, VoiceState};
use songbird::Songbird;

use crate::{
    get_bot_data,
    util::{
        bird::{bird_enqueue, bird_laeve, get_songbird},
        discord::{get_user_read_name, is_human},
    },
    AnyResult,
};

pub async fn handle_voice_state_update(
    ctx: &Context,
    old: &Option<VoiceState>,
    new: &VoiceState,
) -> AnyResult<()> {
    let Some(guild_id) = new.guild_id else {
        return Ok(());
    };

    let manager = get_songbird(ctx).await?;
    let Some(call) = manager.get(guild_id) else {
        return Ok(());
    };
    let Some(channel_id) = call.lock().await.current_channel() else {
        return Ok(());
    };

    match (old, new) {
        (
            Some(VoiceState {
                channel_id: Some(old_channel_id),
                ..
            }),
            VoiceState {
                channel_id: None,
                member: Some(member),
                ..
            },
        ) if channel_id == (*old_channel_id).into() => {
            event(Event::Leave, ctx, manager.clone(), member, guild_id).await?;
        }
        (
            old,
            VoiceState {
                channel_id: Some(new_channel_id),
                member: Some(member),
                ..
            },
        ) if old
            .as_ref()
            .is_none_or(|old| old.channel_id.is_none_or(|v| channel_id != v.into()))
            && channel_id == (*new_channel_id).into() =>
        {
            event(Event::Join, ctx, manager.clone(), member, guild_id).await?;
        }
        _ => {}
    }

    let channel = ctx.http.get_channel(channel_id.0.into()).await?;
    let Channel::Guild(guild) = channel else {
        return Ok(());
    };

    let members = guild.members(&ctx.cache)?;

    let exists_human = members.iter().any(|v| is_human(&v.user));
    if exists_human {
        return Ok(());
    }

    let bot_data = get_bot_data(ctx).await;

    bird_laeve(manager, bot_data.tts_channel.clone(), guild_id).await?;

    Ok(())
}

enum Event {
    Join,
    Leave,
}

async fn event(
    event: Event,
    ctx: &Context,
    manager: Arc<Songbird>,
    member: &Member,
    guild_id: GuildId,
) -> AnyResult<()> {
    let bot_data = get_bot_data(ctx).await;

    if !is_human(&member.user) {
        return Ok(());
    }

    let name = get_user_read_name(member);
    match event {
        Event::Join => tracing::info!("join event of {}", name),
        Event::Leave => tracing::info!("leave event of {}", name),
    }

    let speaker_id = bot_data.get_spekar_id(guild_id, member.user.id).await;

    let audio = match event {
        Event::Leave => {
            bot_data
                .config
                .voice
                .leave
                .process(bot_data.clone(), guild_id, speaker_id, &[name])
                .await?
        }
        Event::Join => {
            bot_data
                .config
                .voice
                .join
                .process(bot_data.clone(), guild_id, speaker_id, &[name])
                .await?
        }
    };

    bird_enqueue(manager, guild_id, audio).await?;

    Ok(())
}
