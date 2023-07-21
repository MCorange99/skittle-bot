
use futures::{future::BoxFuture, FutureExt};
use serenity::{prelude::Context, model::prelude::Message};
use color_eyre::Result;

use crate::modules::{SkittleModuleCommand, SkittleModuleCommandBuilder};

pub fn register() -> SkittleModuleCommand {
    SkittleModuleCommandBuilder::new(exec)
        .help(vec![
            ("ping", "check if bot is responsive")
        ])
        .dev_only(false)
        .required_user_permissions(vec![])
        .required_bot_permissions(vec![])
        .build()
}



pub fn exec(ctx: Context, msg: Message, _: Vec<String>) -> BoxFuture<'static, Result<()>>  {
    async move {
        msg.reply_ping(ctx, "Pong!").await?;
        Ok(())
    }.boxed()
}