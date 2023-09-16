

use futures::{future::BoxFuture, FutureExt};
use serenity::{prelude::Context, model::prelude::Message};
use color_eyre::Result;

use crate::modules::{SkittleModuleCommand, SkittleModuleCommandBuilder};

pub fn register() -> SkittleModuleCommand {
    SkittleModuleCommandBuilder::new(exec)
        .help(vec![
            ("mute [user] [length]", "Mutes a user without a reason Eg. mute @mcorane 1d"),
            ("mute [user] [length] [reason]", "Mutes a user with a reason Eg. mute @mcorane 1d \"Lol y not\""),
        ])
        .dev_only(false)
        .required_user_permissions(vec![

        ])
        .required_bot_permissions(vec![

        ])
        .build()
}



pub fn exec(ctx: Context, msg: Message, args: Vec<String>) -> BoxFuture<'static, Result<()>>  {
    async move {

        // No user
        if args.len() < 2 {
            msg.reply_ping(&ctx.http, "No user provided").await?;
            return Ok(());
        }

        if args.len() < 3 {
            msg.reply_ping(&ctx.http, "No length provided").await?;
            return Ok(());
        }

        let user = &msg.mentions[0];
        let len = crate::util::get_time_len(args[2].clone())?;

        // No reason
        if args.len() < 4 {
            let guild_members = msg.guild_id.unwrap().members(&ctx.http, None, None).await?;
            


            for mut guild_member in guild_members.into_iter(){
                if guild_member.user.id == user.id {
                    guild_member.disable_communication_until_datetime(&ctx.http, len).await?;
                    msg.reply_ping(&ctx.http, format!("Muted user {guild_member} for reason \"None\" untill {len}({})", args[2])).await?;
                    break;
                }
            }

        } else {
            let reason = args[3].clone();
            let guild_members = msg.guild_id.unwrap().members(&ctx.http, None, None).await?;
            


            for mut guild_member in guild_members.into_iter(){
                if guild_member.user.id == user.id {
                    guild_member.disable_communication_until_datetime(&ctx.http, len).await?;
                    msg.reply_ping(&ctx.http, format!("Muted user {guild_member} for reason {reason:?} untill {len}({})", args[2])).await?;
                    break;
                }
            }

        }
        Ok(())
    }.boxed()
}


