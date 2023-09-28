use color_eyre::Result;
use futures::{future::BoxFuture, FutureExt};
use serenity::{prelude::*, model::prelude::*};
use diesel::RunQueryDsl;

use crate::db::models;


pub fn member_add(ctx: Context, member: Member) -> BoxFuture<'static, Result<()>> {
    async move {
        log::debug!("Start");
        use crate::db::schema::core_users;

        let val = models::CoreUsers{
            user_id: member.user.id.0 as i64,
            user_role: 0,
    };

        let mut conn = get_db!(ctx);

        diesel::insert_into(core_users::table)
            .values(&val)
            .execute(&mut conn)?;
        log::debug!("Done");
        Ok(())
    }.boxed()
}