use poise::serenity_prelude::{
    ComponentInteractionDataKind, Context, CreateInteractionResponse,
    CreateInteractionResponseMessage, Interaction,
};

use crate::{
    get_bot_data,
    util::vvc::{VOICE_CHARACTER, VOICE_SELECT_MENU_CUSTOM_ID},
    AnyResult,
};

pub async fn interaction_create(ctx: &Context, interaction: &Interaction) -> AnyResult<()> {
    let Some(component) = interaction.as_message_component() else {
        return Ok(());
    };

    let Some(guild_id) = component.guild_id else {
        return Ok(());
    };

    if !component
        .data
        .custom_id
        .starts_with(VOICE_SELECT_MENU_CUSTOM_ID)
    {
        return Ok(());
    };

    let ComponentInteractionDataKind::StringSelect { values } = &component.data.kind else {
        return Ok(());
    };

    let Some(selected) = values.first() else {
        return Ok(());
    };

    let selected = selected.parse::<u32>()?;

    let Some((name, _)) = VOICE_CHARACTER.iter().find(|v| v.1 == selected) else {
        return Ok(());
    };

    let data = &get_bot_data(ctx).await.speaker_dict;
    data.set(guild_id, component.user.id, selected).await;

    let message = CreateInteractionResponseMessage::new().content(format!(
        "<@{}>さんの声を`{}`に変更しました。",
        component.user.id, name
    ));
    let builder = CreateInteractionResponse::Message(message);

    component.create_response(&ctx.http, builder).await?;

    Ok(())
}
