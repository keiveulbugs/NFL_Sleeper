use crate::Error;

#[poise::command(slash_command)]
pub async fn register(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;

    Ok(())
}
