
use futures::{future::BoxFuture, FutureExt};
use serenity::{prelude::Context, model::prelude::Message};
use color_eyre::Result;

use crate::modules::{SkittleModuleCommand, SkittleModuleCommandBuilder};

pub fn register() -> SkittleModuleCommand {
    SkittleModuleCommandBuilder::new(exec)
        .help(vec![
            ("play [url]", "Plays a song")
        ])
        .dev_only(false)
        .required_user_permissions(vec![])
        .required_bot_permissions(vec![])
        .build()
}



pub fn exec(ctx: Context, msg: Message, _: Vec<String>) -> BoxFuture<'static, Result<()>>  {
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    // ! Keep all of the code inside this async move
    // !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
    async move {


        //* Lock for CoreData rwlock
        //  let cd_lock = {
        //      let data_read = ctx.data.read().await;
        //      data_read.get::<CoreData>().unwrap().clone()
        //  };
        //
        
        //* A read only copy of COre data. can be available globaly in the command
        // let cd_ro = {
        //     cd_lock.read().await.clone()
        // };

        //* A mutable reference to CoreData, unlock this ASAP by putting the code in a scope (by using { }) and keep the code as small as possible in the scope
        // let cd_rw = cd_lock.write().await;

        //* Reply to user with ping
        msg.reply_ping(ctx, "UWU").await?;
        Ok(())
    }.boxed()
}

// async fn join(ctx: &Context, msg: &Message) -> Result<()> {
//     let guild = msg.guild(&ctx.cache).unwrap();
//     let guild_id = guild.id;

//     let channel_id = guild
//         .voice_states.get(&msg.author.id)
//         .and_then(|voice_state| voice_state.channel_id);

//     let connect_to = match channel_id {
//         Some(channel) => channel,
//         None => {
//             msg.reply_ping(&ctx.http, "Not in a voice channel").await?;
//             return Ok(());
//         }
//     };

//     let manager = songbird::get(ctx).await
//         .expect("Songbird Voice client placed in at initialisation.").clone();

//     let _handler = manager.join(guild_id, connect_to).await;

//     Ok(())
// }

// async fn leave(ctx: &Context, msg: &Message) -> Result<()> {
//     let guild = msg.guild(&ctx.cache).unwrap();
//     let guild_id = guild.id;

//     let manager = songbird::get(ctx).await
//         .expect("Songbird Voice client placed in at initialisation.").clone();
//     let has_handler = manager.get(guild_id).is_some();

//     if has_handler {
//         if let Err(e) = manager.remove(guild_id).await {
//             log::error!("Failed {:?}", e);
//         }

//         msg.reply_ping(&ctx.http, "Left the voice channel").await?;
//     } else {
//         msg.reply_ping(&ctx.http, "Not in a voice channel").await?;
//     }

//     Ok(())
// }