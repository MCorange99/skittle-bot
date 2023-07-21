
use futures::{future::BoxFuture, FutureExt};
use serenity::{prelude::Context, model::prelude::Message};
use color_eyre::Result;

pub const HELP: &'static str = "ping (check if bot is responsive)";

pub fn exec(ctx: Context, msg: Message, _: Vec<String>) -> BoxFuture<'static, Result<()>>  {
    async move {
        msg.reply_ping(ctx, "Pong!").await?;
        Ok(())
    }.boxed()
}