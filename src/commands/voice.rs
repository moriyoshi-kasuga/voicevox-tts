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
    let components: Vec<CreateActionRow> = VOICE_CHARACTER
        .chunks(20)
        .enumerate()
        .map(|(i, v)| {
            let options: Vec<CreateSelectMenuOption> = v
                .iter()
                .map(|v| CreateSelectMenuOption::new(v.0, v.1.to_string()))
                .collect();

            let select_menu = CreateSelectMenu::new(
                format!("{}-{}", VOICE_SELECT_MENU_CUSTOM_ID, i),
                CreateSelectMenuKind::String { options },
            );

            CreateActionRow::SelectMenu(select_menu)
        })
        .collect();

    let replay = CreateReply::default()
        .ephemeral(true)
        .components(components);

    ctx.send(replay).await?;
    Ok(())
}
