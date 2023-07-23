
use futures::{future::BoxFuture, FutureExt};
use serenity::{prelude::Context, model::prelude::Message};
use color_eyre::Result;

use crate::modules::{SkittleModuleCommand, SkittleModuleCommandBuilder};

pub fn register() -> SkittleModuleCommand {
    SkittleModuleCommandBuilder::new(exec)
        .help(vec![
            ("command example", "Explanation")  // Various examples and explanations for command
        ])
        .dev_only(false)                        // If the command can only be ran by the bot developers
        .required_user_permissions(vec![])      // The permisions that a user has to have for the command to work
        .required_bot_permissions(vec![])       // The permisions the bot has to have for the command to work
        .build() // Return the built command
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