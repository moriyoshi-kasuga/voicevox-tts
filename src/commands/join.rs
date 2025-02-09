use crate::{
    util::{
        bird::{bird_join, get_songbird},
        get_tts_channel,
    },
    AnyResult, Context,
};

#[poise::command(slash_command, guild_only)]
pub(crate) async fn join(ctx: Context<'_>) -> AnyResult<()> {
    let (guild_id, channel_id) = {
        let Some(guild) = ctx.guild() else {
            ctx.reply("Guild内でしか使えません").await?;
            return Ok(());
        };

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
    let tts_channel = get_tts_channel(serenity_context).await?;

    bird_join(manager, tts_channel, guild_id, channel_id, ctx.channel_id()).await?;

    ctx.say("Voice channel に接続しました！").await?;

    Ok(())
}
