use crate::{
    commands::only_guild,
    get_bot_data,
    util::bird::{bird_join, get_songbird},
    AnyResult, Context,
};

/// Voice channel に入ります
#[poise::command(slash_command, guild_only, aliases("sjoin"))]
pub(crate) async fn join(ctx: Context<'_>) -> AnyResult<()> {
    let (guild_id, channel_id) = {
        only_guild!(ctx, guild);

        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

    let channel_id = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.reply("Voice channel に入ってから実行してください")
                .await?;
            return Ok(());
        }
    };

    let serenity_context = ctx.serenity_context();

    let manager = get_songbird(serenity_context).await?;
    let bot_data = get_bot_data(serenity_context).await;

    bird_join(
        manager,
        bot_data.tts_channel.clone(),
        guild_id,
        channel_id,
        ctx.channel_id(),
    )
    .await?;

    ctx.say("Voice channel に接続しました！").await?;

    Ok(())
}
