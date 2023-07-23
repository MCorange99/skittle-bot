mod commands;

use crate::modules::SkittleModule;



pub fn register() -> SkittleModule {
    let mut module = SkittleModule::register("example module");          // Module Name
    module.set_author("Jeff");                                           // Your name
    module.set_version("0.0.1");                                         // Module version
    module.set_description("Does stuff");                                // Module description in a couple words

    module.register_command("example_command",
        commands::example_command::register());                          // register a command


    module.clone()
}