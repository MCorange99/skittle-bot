mod commands;

use crate::modules::SkittleModule;



pub fn register() -> SkittleModule {
    let mut module = SkittleModule::register("moderation");          // Module Name
    module.set_author("mcorange");                                           // Your name
    module.set_version("0.0.1");                                         // Module version
    module.set_description("Moderation features");                                // Module description in a couple words

    module.register_command("mute",
        commands::mute::register());

    module.register_command("unmute",
        commands::unmute::register());


    module.clone()
}