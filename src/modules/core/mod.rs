mod commands;
mod events;

use crate::modules::SkittleModule;



pub fn register() -> SkittleModule {
    let mut module = SkittleModule::register("core");
    module.set_author("mcorange");
    module.set_version("0.0.1");
    module.set_description("The core utils for skittle bot");


    module.register_command("ping",   commands::ping::register());
    module.register_command("modules",commands::modules::register());
    module.register_command("help",   commands::help::register());
    module.register_command("database",   commands::database::register());

    module.register_event_handler(events::handler);

    module.clone()
}