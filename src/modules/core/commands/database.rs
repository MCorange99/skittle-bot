use clap::Parser;
use color_eyre::Result;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use futures::{future::BoxFuture, FutureExt};
use serenity::{
    model::prelude::{Member, Message, UserId},
    prelude::Context,
};

use crate::{
    db::models,
    modules::{SkittleModuleCommand, SkittleModuleCommandBuilder},
};

pub fn register() -> SkittleModuleCommand {
    SkittleModuleCommandBuilder::new(exec)
        .description("Manage the database")
        .help(vec![("database", "asd")])
        .dev_only(true)
        .required_user_roles(vec![])
        .required_bot_permissions(vec![])
        .build()
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(
        short= 's',
        long = "scan_for_new_users",
        help = "Scans the server for new users and adds them for the new database."
    )]
    scan_for_new_users: bool,
}

pub fn exec(ctx: Context, msg: Message, args: Vec<String>) -> BoxFuture<'static, Result<()>> {
    async move {
        let args = try_parse_args!(Args, msg, ctx, args);

        // msg.reply_ping(ctx, "Pong!").await?;

        if args.scan_for_new_users {
            let gid = match msg.guild_id {
                Some(gid) => gid,
                None => {
                    internal_error!(msg, ctx, "Could not get guild id");
                }
            };

            let guild = ctx.http.get_guild(gid.0).await?;

            ctx.http.get_guilds(None, None).await?;

            let members = get_guild_members!(ctx, guild);

            let current_members = members
                .into_iter()
                .map(|m| models::CoreUsers {
                    user_id: m.user.id.0 as i64,
                    user_role: 0,
                })
                .collect::<Vec<models::CoreUsers>>(); 
            let db_users = {
                let mut conn = get_db!(ctx);
                use crate::db::schema::core_users::dsl::*;
                // use crate::db::schema::core_users::dsl::core_users;
                core_users
                    .select(models::CoreUsers::as_select())
                    .load(&mut conn)
            }?;

            let mut count = 0;

            for member in current_members {
                if (&db_users).into_iter().find(|f| {f.user_id == member.user_id}).is_none() {
                    let mut conn = get_db!(ctx);
                    {
                        use crate::db::schema::core_users;
                        diesel::insert_into(core_users::table)
                            .values(&member)
                            .execute(&mut conn)?;
                    }
                    count += 1;
                } 
            }

            msg.reply_ping(&ctx.http, format!("Added users: {}", count)).await?;

            // log::debug!("{:?}",info);
        }

        Ok(())
    }
    .boxed()
}
