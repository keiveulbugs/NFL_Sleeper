use crate::Error;
use poise::serenity_prelude::{self as serenit, ChannelId};
use serenity::utils::Colour;

/// About command
#[poise::command(slash_command)]
pub async fn help(ctx: poise::Context<'_, (), Error>) -> Result<(), Error> {
    ctx.send(|b| {
        b.embed(|b| b.description(
            "This bot fetches data from Sleeper.\nCreated by <@397118394714816513>💕"
            ).title("help").colour(Colour::BLITZ_BLUE))
            .ephemeral(true)
            .components(|b| {
                b.create_action_row(|b| {
                    b
                    .create_button(|b| {
                        b.label("Support / Custom bot requests")
                            .url("https://discord.gg/uUVNbzQFDE")
                            .style(serenit::ButtonStyle::Link)})
                    .create_button(|b|{
                        b.label("Invite bot")
                            .url("https://discord.com/api/oauth2/authorize?client_id=1069277136017436782&permissions=2147502080&scope=bot")
                            .style(serenit::ButtonStyle::Link)
                    })
                })
            })
    })
    .await?;
    //When the message is sent in your private channel, return the option to deregister the bot.
    if ctx.channel_id() == ChannelId(1028693862786547782) {
        poise::builtins::register_application_commands_buttons(ctx).await?;
    }
    Ok(())
}
