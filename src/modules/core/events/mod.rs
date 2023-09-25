mod member_add;

use futures::{future::BoxFuture, FutureExt};
use serenity::prelude::Context;
use crate::modules::types::EventType;
use color_eyre::Result;

pub fn handler(ctx: Context, event: EventType) -> BoxFuture<'static, Result<()>> {

    async move {
        match event {
            EventType::guild_member_addition(e) => member_add::member_add(ctx, e).await?,
            _ => ()
        }
        Ok(())
    }.boxed()
}