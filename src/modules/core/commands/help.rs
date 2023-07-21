
use futures::{future::BoxFuture, FutureExt};
use serenity::{prelude::Context, model::{prelude::Message, Timestamp}};
use color_eyre::Result;

use crate::{CoreData, modules::{SkittleModuleCommand, SkittleModuleCommandBuilder}};


pub fn register() -> SkittleModuleCommand {
    SkittleModuleCommandBuilder::new(exec)
        .help(vec![
            ("help",            "(shows overview of all commands)"),
            ("help [command]", " (shows help for a specific command)")
        ])
        .dev_only(false)
        .required_user_permissions(vec![])
        .required_bot_permissions(vec![])
        .build()
}


pub fn exec(ctx: Context, msg: Message, args: Vec<String>) -> BoxFuture<'static, Result<()>>  {
    async move {
        let cd_lock = {
            let data_read = ctx.data.read().await;
            data_read.get::<CoreData>().unwrap().clone()
        };
        log::debug!("1");
        if args.len() < 2 {

            let mut helps = Vec::new();
            log::debug!("2");

            let cd = {cd_lock.read().await.clone()};

            for (modl_name, modl) in cd.modules {
                log::debug!("3");
                let mut module_helps = String::new();
                
                for comm in modl.commands {
                    module_helps.push_str(format!("{}\n", comm.0).as_str())
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
        } else {

            let cd = {cd_lock.read().await.clone()};
            let mut helps = String::new();


            'outer: for (module_name, mut module) in cd.modules {
                for (k, fun) in module.commands() {
                    if k == args[1] {

                        for h in fun.help {

                            helps.push_str(format!("{} - {}\n", h.0, h.1).as_str());
                        }
                        

                        msg
                            .channel_id
                            .send_message(&ctx.http, |m| {
                                m
                                .embed(|e| {
                                    e.title(format!("Help for {}::{}", module_name, args[1]))
                                        .field(args[1].clone(), helps, false)
                                        .footer(|f| f.text("UwU"))
                                        .timestamp(Timestamp::now())
                                })
                        })
                        .await?;   




                        break 'outer;
                    }
                }
            }
        }

        Ok(())
    }.boxed()
}