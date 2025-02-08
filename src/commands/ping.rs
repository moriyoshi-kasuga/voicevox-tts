use crate::{Context, Error};

#[poise::command(slash_command)]
pub(crate) async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;

    Ok(())
}
