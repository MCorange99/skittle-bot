
use serenity::{async_trait, prelude::{EventHandler, Context}, model::prelude::{Message, Ready, Member}};
use crate::{CoreData, modules::{types::EventType, SkittleModule}, db::models::UserRole};
use color_eyre::Result;

#[derive(Debug, Default)]
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        
        let cd_lock = {
            let data_read = ctx.data.read().await;
            data_read.get::<CoreData>().unwrap().clone()
        };

        let prefix = {cd_lock.read().await.config.prefix.clone()};

        if msg.content.trim().starts_with(&prefix) {
            
            // if let Err(why) = msg.channel_id.say(&ctx.http, format!("Parsed: {argv:#?}")).await {
                //     log::error!("Failed to send message: {why:?}");
                //     return;
                // }
                
                
                
            let cdata = {cd_lock.read().await.clone()};
            'outer: for (_, module) in cdata.modules.clone() {
                
                
                //? parse args
                let Ok(argv) = shellwords::split(msg.content.trim().strip_prefix(&prefix).unwrap().trim()) else {
                    let _ = msg.channel_id.say(&ctx.http, crate::locale::en_US::handler::failed_parsing(&msg.content)).await;
                    log::error!("Failed to parse command {:?} by {}", msg.content,  msg.author.id);
                    break 'outer;
                };

                if argv.is_empty() {
                    break 'outer;
                }
                
                log::debug!("User {} ran a command: {:?}", msg.author.id, argv);

                let mut module = module as SkittleModule;
                for (k, command) in module.commands() {
                    if k == argv[0] {



                        //? User auth checks
                        {
                            let is_dev = cdata.config.users.developers.contains(&format!("{}", msg.author.id.0));

                            if command.dev_only && !is_dev {
                                let _ = msg.reply_ping(&ctx.http, crate::locale::en_US::handler::NOT_DEVELOPER).await;
                            }

                            //? Allow devs to bypass permissions

                            if !is_dev {

                                use crate::db::models::CoreUsers;
                                let user: CoreUsers = {
                                let mut conn = get_db!(ctx);
                                use crate::db::schema::core_users::dsl::core_users;
                                use diesel::*;
                        
                                match {
                                    core_users
                                        .select(CoreUsers::as_select())
                                        .find(msg.author.id.0 as i64)
                                        .first(&mut conn)
                                } {
                                    Ok(r) => r,
                                    Err(e) => {
                                        let _ = msg.reply_ping(&ctx.http, crate::locale::en_US::INTERNAL_ERROR).await;
                                        log::error!("Internal Error: {:?}", e);
                                        break 'outer;
                                    }

                                }
                                };

                                
                                let mut missing_roles = Vec::new();

                                for role in command.required_user_roles {
                                    if !UserRole::has(user.user_role, role) {
                                        missing_roles.push(role);
                                    }
                                }
                                
                                
                                if !missing_roles.is_empty() {
                                    let _ = msg.reply_ping(&ctx.http, crate::locale::en_US::handler::missing_roles(missing_roles)).await;
                                    break 'outer;
                                }
                            } else if !command.required_user_roles.is_empty() {
                                log::info!("Developer {id} bypassed required roles: {perms:?}", id=msg.author.id.0, perms=command.required_user_roles)
                            }


                        }

                        //? User auth checks end
                        
                        if let Err(e) = (command.exec)(ctx.clone(), msg.clone(), argv).await {
                            log::warn!("Failed to execute command: {e}");
                        };
                        break 'outer;
                    }
                }
            }
        }
        // todo: handle this result
        send_event(&ctx, EventType::message(msg)).await.unwrap();

    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        log::info!("{} is connected!", ready.user.name);
    }

    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        send_event(&ctx, EventType::guild_member_addition(member)).await.unwrap();
    }

}

pub async fn send_event(ctx: &Context, event: EventType) -> Result<()> {

    log::debug!("Sent event {:?}", event.to_string());
    let cd_lock = {
        let ctx2 = ctx.clone();
        let data_read = ctx2.data.read().await;
        data_read.get::<CoreData>().unwrap().clone()
    };

    let cdata = {cd_lock.read().await.clone()};
    for (_, module) in cdata.modules.clone() {
        let mut module: SkittleModule = module;
        match module.event_handler() {
            Some(eh)=> {
                eh(ctx.clone(), event.clone()).await?;
            }
            None => ()
        }
    }

    Ok(())
}