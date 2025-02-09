use crate::{
    commands::only_guild,
    get_bot_data,
    util::bird::{bird_laeve, get_songbird},
    AnyResult, Context,
};

/// Voice channel から切断します
#[poise::command(slash_command, guild_only, aliases("slaeve"))]
pub(crate) async fn leave(ctx: Context<'_>) -> AnyResult<()> {
    only_guild!(ctx, guild_id);

    let manager = get_songbird(ctx.serenity_context()).await?;

    if manager.get(guild_id).is_some() {
        let bot_data = get_bot_data(ctx.serenity_context()).await;
        let tts_channel = &bot_data.tts_channel;

        if let Err(e) = bird_laeve(manager, tts_channel.clone(), guild_id).await {
            ctx.say(format!("Failed please retry: {:?}", e)).await?;
        }
        ctx.say("Voice channel から切断しました").await?;
    } else {
        ctx.say("Voice channel に入っていません").await?;
    }

    Ok(())
}
