use serenity::{prelude::Context, model::prelude::Message};
use anyhow::Result;

#[no_mangle]
pub async fn exec(ctx: Context, msg: Message, _: Vec<String>) -> Result<()> {
    msg.reply_mention(ctx, "Pong!").await?;
    Ok(())
}