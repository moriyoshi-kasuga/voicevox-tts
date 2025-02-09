use poise::{
    serenity_prelude::{
        CreateActionRow, CreateSelectMenu, CreateSelectMenuKind, CreateSelectMenuOption,
    },
    CreateReply,
};

use crate::{
    util::vvc::{VOICE_CHARACTER, VOICE_SELECT_MENU_CUSTOM_ID},
    AnyResult, Context,
};

/// 音声の声を切り替えます
#[poise::command(slash_command, guild_only, aliases("svoice"))]
pub(crate) async fn voice(ctx: Context<'_>) -> AnyResult<()> {
    let options = VOICE_CHARACTER
        .iter()
        .map(|v| CreateSelectMenuOption::new(v.0, v.1.to_string()))
        // TODO:
        .take(20)
        .collect();

    let select_menu = CreateSelectMenu::new(
        VOICE_SELECT_MENU_CUSTOM_ID,
        CreateSelectMenuKind::String { options },
    );

    let component = CreateActionRow::SelectMenu(select_menu);

    let replay = CreateReply::default()
        .ephemeral(true)
        .components(vec![component]);

    ctx.send(replay).await?;
    Ok(())
}
