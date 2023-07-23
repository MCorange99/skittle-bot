use std::collections::HashMap;

#[allow(unused_imports)]
use crate::{modules, CoreData};
use color_eyre::Result;

use crate::modules::SkittleModule;

#[allow(unused_mut, unused_variables)]
pub fn load(cd: &mut CoreData) -> Result<()> {
    log::info!("Loading modules");
    let mut commands: HashMap<String, String> = HashMap::new();

    #[cfg(feature = "core")]
    load_module(cd, &mut commands, modules::core::register());
    
    load_module(cd, &mut commands, modules::moderation::register());


    Ok(())
}

#[allow(dead_code)]
fn load_module(cd: &mut CoreData, commands: &mut HashMap<String, String>, mut module: SkittleModule) {
    if !cd.available_modules.contains(&module.name()){
        cd.available_modules.push(module.name());
    }
    if cd.config.modules.disabled_modules.contains(&module.name()) {
        log::error!("Disabled module {} not loaded", module.name());
        return;
    }
    log::info!("Loading module {}", module.name());
    for comm in module.commands() {           
        if commands.contains_key(&comm.0) {
            let first = commands.get(&comm.0).unwrap();
            log::error!("A duplicate command was attempted to register (First: {}::{}) (Second: {}::{})", first, comm.0, module.name(), comm.0);
        }


        log::info!("Registered command: {}::{}", module.name(), comm.0);
        commands.insert(comm.0, module.name());
        break;
    }
    cd.modules.insert(module.name(), module);
}