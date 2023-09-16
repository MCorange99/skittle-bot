

use futures::{future::BoxFuture, FutureExt};
use serenity::{prelude::Context, model::prelude::Message};
use color_eyre::Result;

use crate::modules::{SkittleModuleCommand, SkittleModuleCommandBuilder};

pub fn register() -> SkittleModuleCommand {
    SkittleModuleCommandBuilder::new(exec)
        .help(vec![
            ("unmute [user]", "Unmutes a user without a reason Eg. unmute @mcorane"),
            ("unmute [user] [reason]", "Unmutes a user with a reason Eg. unmute @mcorane \"Lol y not\""),
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

        let user = &msg.mentions[0];

        // No reason
        if args.len() < 4 {
            let guild_members = msg.guild_id.unwrap().members(&ctx.http, None, None).await?;

            for mut guild_member in guild_members.into_iter(){
                if guild_member.user.id == user.id {
                    guild_member.enable_communication(&ctx.http).await?;
                    msg.reply_ping(&ctx.http, format!("Unmuted user {guild_member} for reason \"None\"")).await?;
                    break;
                }
            }

        } else {
            let reason = args[2].clone();
            let guild_members = msg.guild_id.unwrap().members(&ctx.http, None, None).await?;
            
            for mut guild_member in guild_members.into_iter(){
                if guild_member.user.id == user.id {
                    guild_member.enable_communication(&ctx.http).await?;
                    msg.reply_ping(&ctx.http, format!("Unuted user {guild_member} for reason {reason:?}")).await?;
                    break;
                }
            }

        }
        Ok(())
    }.boxed()
}


