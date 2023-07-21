
use crate::modules::SkittleModule;



pub fn register() -> SkittleModule {
    let mut module = SkittleModule::register("test");
    module.set_author("mcorange");
    module.set_version("0.0.1");
    module.set_description("The core utils for skittle bot");




    module.clone()
}