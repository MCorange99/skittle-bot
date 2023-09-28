
use futures::{future::BoxFuture, FutureExt};
use serenity::{prelude::Context, model::{prelude::Message, Timestamp}};
use color_eyre::Result;

use crate::{CoreData, modules::{SkittleModuleCommand, SkittleModuleCommandBuilder}, read_type_map, get_type_map_data_ro_cloned};



pub fn register() -> SkittleModuleCommand {
    SkittleModuleCommandBuilder::new(exec)
        .help(vec![
            ("modules list",                            "Lists all modules"      ),
            ("modules [module name]",                   "prints module info"     ),
            ("modules [module name] [enable/disable]",  "disables/enables module"),
        ])
        .dev_only(true)
        .required_user_roles(vec![])
        .required_bot_permissions(vec![])
        .build()
}


pub fn exec(ctx: Context, msg: Message, args: Vec<String>) -> BoxFuture<'static, Result<()>>  {
    async move {
        {
            let cd_ro = get_type_map_data_ro_cloned!(CoreData, ctx);

            if args.len() < 2 {
                msg.reply_ping(ctx, "No subcommand provided").await?;
                return Ok(());
            }

            match args[1].as_str() {

                "list" => {
                    let mut module_data = vec![];

                    for (k, v) in &get_type_map_data_ro_cloned!(CoreData, ctx).modules {
                        let mut v = v.clone();
                        module_data.push((format!("**{}**", k.clone().to_uppercase()), format!("**Description**: {}\n**Version**: {}\n**Author**: {}\n**Commands**: {}", v.description(), v.version(), v.author(), v.commands().len()),false))
                    }

                    msg
                    .channel_id
                    .send_message(&ctx.http, |m| {
                        m
                        .embed(|e| {
                            e.title("Modules")
                                .description("All enabled modules")
                                .fields(module_data.clone())
                                .field("Available Modules", cd_ro.available_modules.join("\n"), false)
                                .field("Disabled Modules", cd_ro.config.modules.disabled_modules.join("\n"), false)
                                .footer(|f| f.text("UwU"))
                                .timestamp(Timestamp::now())
                        })
                    })
                    .await?;
                }

                s => {
                    let s = s.to_string();
                    {

                        if !cd_ro.available_modules.contains(&s) {
                            msg.reply_ping(ctx, format!("Module {s} not found")).await?;
                            return Ok(());
                        }

                        if args.len() < 3 {
                            log::debug!("{:#?}", cd_ro.modules);
                            let mut modl = cd_ro.modules.get(&s).unwrap().clone();

                            let mut modl_comm_info = Vec::new();

                            for (k, _) in &modl.commands {
                                modl_comm_info.push(format!("{k}"))
                            }


                            msg
                                .channel_id
                                .send_message(&ctx.http, |m| {
                                    m
                                    .embed(|e| {
                                        e.title(format!("Module {}", s))
                                            .field("Commands", modl_comm_info.join("\n"), false)
                                            .field("Description", modl.description(), false)
                                            .field("Version", modl.version(), false)
                                            .field("Author", modl.author(), false)
                                            .field("Commands", modl.commands().len(), false)
                                            .footer(|f| f.text("UwU"))
                                            .timestamp(Timestamp::now())
                                    })
                                })
                                .await?;
                            return Ok(());
                        }
                    }
                }
            }
            
        }

        let cd_lock = read_type_map!(CoreData, ctx);

        let mut cd = cd_lock.write().await;

        if args.len() < 3 {
            return Ok(());
        }

        match args[2].as_str() {
            "enable" | "on" | "1" => {
                if cd.config.modules.disabled_modules.contains(&args[1]) {
                    for (c, m) in cd.config.modules.disabled_modules.clone().iter().enumerate() {
                        if m == &args[1] {
                            cd.config.modules.disabled_modules.remove(c);
                        }
                    }
                    crate::module_loader::load(&mut cd)?;
                    msg.reply_ping(ctx, format!("Module {} enabled", args[1])).await?;
                } else {
                    msg.reply_ping(ctx, format!("Module {} is already enabled", args[1])).await?;
                }
                crate::config::set_core_config(cd.config.clone())?;
            }
            "disable" | "off" | "0" => {
                
                if !cd.config.modules.disabled_modules.contains(&args[1]) {
                    cd.config.modules.disabled_modules.push(args[1].clone());
                    cd.modules.remove(&args[1]);
                    msg.reply_ping(ctx, format!("Module {} disabled", args[1])).await?;
                } else {
                    msg.reply_ping(ctx, format!("Module {} is already disabled", args[1])).await?;
                }
                crate::config::set_core_config(cd.config.clone())?;
            }
            s => {
                msg.reply_ping(ctx, format!("Unknown subcommand {s}")).await?;
                return Ok(());
            }
        }

            


        // msg.reply_mention(ctx, "Pong!").await?;
        Ok(())
    }.boxed()
}