mod config;
mod handler;
mod util;
mod module_loader;
mod modules;

// use skittle_bot_core;


use std::sync::Arc;
use std::collections::HashMap;
use color_eyre::Result;
use serenity::{prelude::{TypeMapKey, GatewayIntents}, Client};
use tokio::{self, sync::RwLock};
use crate::modules::SkittleModule;
#[derive(Debug, Clone)]
pub struct CoreData {
    config: config::CoreConfig,
    modules: HashMap<String, SkittleModule>,
    available_modules: Vec<String>
}

impl TypeMapKey for CoreData {
    // While you will be using RwLock or Mutex most of the time you want to modify data,
    // sometimes it's not required; like for example, with static data, or if you are using other
    // kinds of atomic operators.
    //
    // Arc should stay, to allow for the data lock to be closed early.
    type Value = Arc<RwLock<CoreData>>;
}

#[tokio::main]
async fn main() -> Result<()>{
    dotenv::dotenv()?;
    env_logger::init();
    let mut cdata = CoreData { config:config::get_core_config()?,modules:HashMap::new(), available_modules: vec![] };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    log::info!("Loading modules");
    module_loader::load(&mut cdata)?;
    log::debug!("{cdata:#?}");
    log::info!("Building client");

    let mut client =
        Client::builder(&cdata.config.secrets.token, intents)
            .event_handler(handler::Handler::default())
            .type_map_insert::<CoreData>(Arc::new(RwLock::new(cdata)))
            .await
            .expect("Err creating client");


    log::info!("Starting client");
    if let Err(why) = client.start().await {
        log::error!("Client error: {:?}", why);
    }

    Ok(())
}
