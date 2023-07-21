
use futures::{future::BoxFuture, FutureExt};
use serenity::{prelude::Context, model::prelude::Message};
use anyhow::Result;

pub fn exec(ctx: Context, msg: Message, _: Vec<String>) -> BoxFuture<'static, Result<()>>  {
    async move {
        msg.reply_mention(ctx, "Pong!").await?;
        Ok(())
    }.boxed()
}