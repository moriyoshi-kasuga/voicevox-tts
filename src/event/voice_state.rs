use poise::serenity_prelude::{Channel, Context, GuildId};

use crate::{util::bird::get_songbird, AnyResult};

pub async fn handle_voice_state_update(ctx: &Context, guild_id: Option<GuildId>) -> AnyResult<()> {
    let Some(guild_id) = guild_id else {
        return Ok(());
    };

    let manager = get_songbird(ctx).await?;
    let Some(call) = manager.get(guild_id) else {
        return Ok(());
    };
    let Some(channel_id) = call.lock().await.current_channel() else {
        return Ok(());
    };

    let channel = ctx.http.get_channel(channel_id.0.into()).await?;
    let Channel::Guild(guild) = channel else {
        return Ok(());
    };

    let members = guild.members(&ctx.cache)?;

    let exists_human = members.iter().any(|v| !v.user.bot && !v.user.system);
    if exists_human {
        return Ok(());
    }

    manager.remove(guild_id).await?;

    Ok(())
}
