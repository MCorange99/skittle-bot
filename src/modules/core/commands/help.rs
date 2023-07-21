
use futures::{future::BoxFuture, FutureExt};
use serenity::{prelude::Context, model::{prelude::Message, Timestamp}};
use color_eyre::Result;

use crate::CoreData;

pub const HELP: &'static str = "help (shows overview of all commands)\nhelp [command] (shows help for a specific command)";

pub fn exec(ctx: Context, msg: Message, args: Vec<String>) -> BoxFuture<'static, Result<()>>  {
    async move {
        let cd_lock = {
            let data_read = ctx.data.read().await;
            data_read.get::<CoreData>().unwrap().clone()
        };
        if args.len() < 2 {

            let mut helps = Vec::new();

            for (modl_name, modl) in &cd_lock.read().await.modules {

                let mut module_helps = String::new();
                
                for (k, v) in &modl.commands {
                    module_helps.push_str(format!("**{k}**\n{}\n", v.1).as_str());
                }
                helps.push((modl_name.to_uppercase(), module_helps, false));
            }


            msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m
                    .embed(|e| {
                        e.title("Help")
                            .fields(helps)
                            // .field("Commands", modl_comm_info.join("\n"), false)
                            .footer(|f| f.text("UwU"))
                            .timestamp(Timestamp::now())
                    })
                })
                .await?;
        }

        Ok(())
    }.boxed()
}