
use futures::{future::BoxFuture, FutureExt};
use serenity::{prelude::Context, model::{prelude::Message, Timestamp}};
use color_eyre::Result;
use clap::Parser;
use crate::{CoreData, modules::{SkittleModuleCommand, SkittleModuleCommandBuilder}, get_type_map_data_ro_cloned, try_parse_args};


pub fn register() -> SkittleModuleCommand {
    SkittleModuleCommandBuilder::new(exec)
        .help(vec![
            ("help",            "(shows overview of all commands)"),
            ("help [command]", " (shows help for a specific command)")
        ])
        .dev_only(false)
        .required_user_roles(vec![])
        .required_bot_permissions(vec![])
        .build()
}

#[derive(Debug, Parser)]
struct Args {
    subcommands: Vec<String>
}


pub fn exec(ctx: Context, msg: Message, args: Vec<String>) -> BoxFuture<'static, Result<()>>  {
    async move {

        let args = try_parse_args!(Args, msg, ctx, args);

        

        if args.subcommands.len() < 2 {

            let mut helps = Vec::new();

            let cd = get_type_map_data_ro_cloned!(CoreData, ctx);

            for (modl_name, modl) in cd.modules {
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

            let cd = get_type_map_data_ro_cloned!(CoreData, ctx);
            let mut helps = String::new();


            'outer: for (module_name, mut module) in cd.modules {
                for (k, fun) in module.commands() {
                    if k == args.subcommands[1] {

                        for h in fun.help {

                            helps.push_str(format!("{} - {}\n", h.0, h.1).as_str());
                        }
                        

                        msg
                            .channel_id
                            .send_message(&ctx.http, |m| {
                                m
                                .embed(|e| {
                                    e.title(format!("Help for {}::{}", module_name, args.subcommands[1]))
                                        .field(args.subcommands[1].clone(), helps, false)
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