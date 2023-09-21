use color_eyre::Result;
use serenity::{prelude::*, model::prelude::*};
use diesel::{SelectableHelper, RunQueryDsl};

use crate::{with_db, db::models};


pub async fn member_add(ctx: Context, member: Member) -> Result<()> {
    use crate::db::schema::core_users;

    let val = models::core_users::new_core_users{
        user_id: member.user.id.0 as i64,
    };


    let ret: Result<usize> = with_db!(ctx, |conn| {
        diesel::insert_into(core_users::table)
            .values(&val)
            .execute(conn)
    });

    Ok(())
}