use crate::{util::bird::get_songbird, AnyResult, Context};

#[poise::command(slash_command, guild_only)]
pub(crate) async fn leave(ctx: Context<'_>) -> AnyResult<()> {
    let Some(guild_id) = ctx.guild_id() else {
        ctx.reply("Guild内でしか使えません").await?;
        return Ok(());
    };

    let manager = get_songbird(ctx.serenity_context()).await?;

    if manager.get(guild_id).is_some() {
        if let Err(e) = manager.remove(guild_id).await {
            ctx.say(format!("Failed: {:?}", e)).await?;
        }
        ctx.say("Voice channel から切断しました").await?;
    } else {
        ctx.say("Voice channel に入っていません").await?;
    }

    Ok(())
}
