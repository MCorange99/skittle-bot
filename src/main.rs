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

    let config = config::get_core_config()?;

    logger_init(&config);

    let mut cdata = CoreData {
        config,
        modules: HashMap::new(),
        available_modules: vec![]
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_VOICE_STATES;

    module_loader::load(&mut cdata)?;
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


fn logger_init(config: &CoreConfig) {

    let log_dir = Path::new("./logs");

    if !log_dir.is_dir() {
        let _ = std::fs::DirBuilder::new().create(log_dir);
    }
    
    if config.write_logs_to_file {

        use std::io::Write;
        let target = Box::new(
            File::options()
                .append(true)
                .create(true)
                .open(
                log_dir.join(
                    format!("{}-log.txt", Local::now().format("%Y-%m-%d"))
                )
            ).expect("Can't create file"));
    
        let mut logger = env_logger::Builder::new();
        let logger = logger.target(env_logger::Target::Pipe(target));
    
    
        let logger = if config.debug {
            logger.filter(None, LevelFilter::Debug)
        } else {
            logger.filter(Some("tracing::span"), LevelFilter::Warn)
                .filter(Some("serenity"), LevelFilter::Warn)
                .filter(Some("skittle_bot_core"), LevelFilter::Info)
        };
            
        logger.format(|buf, record| {
                writeln!(
                    buf,
                    "[{} {} {}:{}] {}",
                    Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    record.level(),
                    record.file().unwrap_or("unknown"),
                    record.line().unwrap_or(0),
                    record.args()
                )
            })
            .init();
    } else {
        env_logger::init();
    }

}
