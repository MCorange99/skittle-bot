
use clap::Parser;
use futures::{future::BoxFuture, FutureExt};
use serenity::{prelude::Context, model::prelude::{Message, UserId, Member}};
use color_eyre::Result;
use diesel::{RunQueryDsl, QueryDsl, SelectableHelper};

use crate::{modules::{SkittleModuleCommand, SkittleModuleCommandBuilder}, try_parse_args, internal_error, get_guild_members, with_db, db::models};

pub fn register() -> SkittleModuleCommand {
    SkittleModuleCommandBuilder::new(exec)
        .description("Manage the database")
        .help(vec![
            ("database", "asd")
        ])
        .dev_only(true)
        .required_user_permissions(vec![])
        .required_bot_permissions(vec![])
        .build()
}

#[derive(Debug, Parser)]
struct Args {
    #[arg(long="scan_for_new_users", help="Scans the server for new users and adds them for the new database.")]
    scan_for_new_users: bool
}


pub fn exec(ctx: Context, msg: Message, args: Vec<String>) -> BoxFuture<'static, Result<()>>  {
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
            let info = ctx.http.get_member(gid.0, members[0].user.id.0).await;

            let db_users = members.into_iter().map(|m|{
                models::core_users::new_core_users{
                    user_id: m.user.id.0 as i64,
                }
            }).collect::<Vec<models::core_users::new_core_users>>();

            let count = 
            with_db!(ctx, |conn| {
                use crate::db::schema::core_users::dsl;
                dsl::core_users.load::<models::core_users::core_users>(conn)

            });

            msg.reply_ping(&ctx.http, "");

            // log::debug!("{:?}",info);
        }

        Ok(())
    }.boxed()
}