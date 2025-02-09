use poise::CreateReply;

use crate::{commands::only_guild, util::get_dict, AnyResult, Context};

/// 辞書を操作します
#[poise::command(
    slash_command,
    guild_only,
    aliases("sdict"),
    subcommands("add", "remove", "list")
)]
pub(crate) async fn dict(ctx: Context<'_>) -> AnyResult<()> {
    only_guild!(ctx, guild_id, _guild_id);
    ctx.reply("dict(sdict) `add`, `remove`, `list` のサブコマンドがあります。 ")
        .await?;
    Ok(())
}

/// 辞書にキーで値を追加します
#[poise::command(slash_command)]
pub(crate) async fn add(ctx: Context<'_>, key: String, value: String) -> AnyResult<()> {
    only_guild!(ctx, guild_id);
    let dict = get_dict(ctx.serenity_context()).await?;
    let text = if dict.set(guild_id, key.clone(), value.clone()).await {
        format!("`{}`の読み方を`{}`として辞書に登録しました。", key, value)
    } else {
        format!("すでに`{}`は辞書に登録されています。", key)
    };
    ctx.reply(text).await?;
    Ok(())
}

/// 辞書でキーを削除します
#[poise::command(slash_command)]
pub(crate) async fn remove(ctx: Context<'_>, key: String) -> AnyResult<()> {
    only_guild!(ctx, guild_id);
    let dict = get_dict(ctx.serenity_context()).await?;
    let text = if dict.remove(guild_id, &key).await {
        format!("辞書から`{}`を削除しました。", key)
    } else {
        format!("`{}`は辞書に登録されていません。", key)
    };
    ctx.reply(text).await?;
    Ok(())
}

/// 辞書の中身を全部見ます
#[poise::command(slash_command)]
pub(crate) async fn list(ctx: Context<'_>) -> AnyResult<()> {
    only_guild!(ctx, guild_id);
    let dict = get_dict(ctx.serenity_context()).await?;
    let embed = dict.get_dist(guild_id).await;
    let guild_name = guild_id
        .name(ctx.cache())
        .unwrap_or_else(|| "サーバー".to_string());
    let embed = embed.title(format!("📕 {}の辞書", guild_name));
    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
