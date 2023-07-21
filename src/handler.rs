
use serenity::{async_trait, prelude::{EventHandler, Context}, model::prelude::{Message, Ready}};
use crate::CoreData;

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
       
        let data = ctx.clone();
        let data = data.data.read().await;
        let cdata = data.get::<CoreData>().unwrap().read().await;

        let prefix = &cdata.config.prefix;

        if msg.content.trim().starts_with(prefix) {
            let Ok(argv) = shellwords::split(msg.content.trim().strip_prefix(prefix).unwrap().trim()) else {
                // if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                //     log::error!("Failed to send message: {why:?}");
                //     return;
                // }
                return;
            };

            if let Err(why) = msg.channel_id.say(&ctx.http, format!("Parsed: {argv:#?}")).await {
                log::error!("Failed to send message: {why:?}");
                return;
            }

            'outer: for (_, mut module) in cdata.modules.clone() {
                for fun in module.commands() {
                    if fun.0 == argv[0] {
                        (fun.1)(ctx, msg, argv.clone()).await.unwrap();
                        break 'outer;
                    }
                }
            }
        }


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
}