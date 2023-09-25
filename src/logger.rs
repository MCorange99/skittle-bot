use std::{path::Path, fs::File};

use chrono::Local;
use log::LevelFilter;

use crate::config::CoreConfig;

pub fn init(config: &CoreConfig) {

    
    if config.write_logs_to_file {
        println!("Writing logs to file");
        let log_dir = Path::new("./logs");
    
        if !log_dir.is_dir() {
            let _ = std::fs::DirBuilder::new().create(log_dir);
        }
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
