use color_eyre::Result;
use futures::{FutureExt, future::BoxFuture};
use serenity::{prelude::*, model::prelude::*};
use crate::db::*;

pub fn member_add(ctx: Context, member: Member) -> BoxFuture<'static, Result<()>>  {

    async move {
        let mut val = CoreUser::new(member.user.id.0);        
        val.save(UserInternalId::new(), &mut get_db!(ctx)).await?;
        
        Ok(())
    }.boxed()
}