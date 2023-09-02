mod commands;

use crate::modules::SkittleModule;



pub fn register() -> SkittleModule {
    let mut module = SkittleModule::register("music");
    module.set_author("mcorange");
    module.set_version("0.0.1");
    module.set_description("Music streaming from youtube");

    module.register_command("play",
        commands::play::register());


    module.clone()
}