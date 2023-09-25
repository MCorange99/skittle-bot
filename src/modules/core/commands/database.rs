
use clap::Parser;
use futures::{future::BoxFuture, FutureExt};
use serenity::{prelude::Context, model::prelude::{Message, UserId, Member}};
use color_eyre::Result;

use crate::{modules::{SkittleModuleCommand, SkittleModuleCommandBuilder}, try_parse_args, internal_error, get_guild_members, db::{CoreUser, UserInternalId}};

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

            let current_members = get_guild_members!(ctx, guild);
            // let info = ctx.http.get_member(gid.0, members[0].user.id.0).await;


            let mut db = get_db!(ctx);
            let db_cache = db.get_cached_data();


            let db_member_ids = db_cache.users.iter().map(|(_, v)| v.discord_id).collect::<Vec<u64>>();

            let mut count = 0;

            for member in current_members {
                if !db_member_ids.contains(&member.user.id.0) {
                    count += 1;
                    let id = UserInternalId::new();
                    log::debug!("added, {} {:?}", member.user.id.0, id);
                    CoreUser::new(member.user.id.0).save(id, &mut db).await?;
                }
            }

            db.save_cached_data().await?;

            dbg!(db.get_cached_data());

            // let count = 
            // with_db!(ctx, |conn| {
            //     use crate::db::schema::core_users::dsl;
            //     dsl::core_users.load::<models::core_users::core_users>(conn)

            // });

            msg.reply_ping(&ctx.http, format!("```md\n# Results\nAdded members: {}\n```", count)).await?;
            
            // log::debug!("{:?}",info);

        } else {
            let text = concat!(
                "```\n",
                "error: No command provided\n\n",
                "Usage: database [OPTIONS]\n\n",
                "For more information, try '--help'.\n",
                "```"
            );
        
            msg.reply_ping(&ctx.http, text).await?;
        }
        

        Ok(())
    }.boxed()
    
}