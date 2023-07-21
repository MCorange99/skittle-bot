mod commands;

use crate::modules::SkittleModule;



#[no_mangle]
pub fn register() -> SkittleModule {
    let mut module = SkittleModule::register("core");
    module.set_author("mcorange");
    module.set_version("0.0.1");
    module.set_description("The core utils for skittle bot");


    module.register_command("ping", commands::ping::exec);


    module.clone()
}