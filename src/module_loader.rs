use std::collections::HashMap;

use crate::{modules, CoreData};
use anyhow::Result;

use crate::modules::SkittleModule;

pub fn load(cd: &mut CoreData) -> Result<()> {
    let mut commands: HashMap<String, String> = HashMap::new();

    #[cfg(feature = "core")]
    load_module(cd, &mut commands, modules::core::register());


    Ok(())
}


fn load_module(cd: &mut CoreData, commands: &mut HashMap<String, String>, mut module: SkittleModule) {
    for comm in module.commands() {           
        if commands.contains_key(&comm.0) {
            let first = commands.get(&comm.0).unwrap();
            log::error!("A duplicate command was attempted to register (First: {}::{}) (Second: {}::{})", first, comm.0, module.name(), comm.0);
        }
        log::info!("Registered command: {}::{}", module.name(), comm.0);
        commands.insert(comm.0, module.name());
        cd.modules.insert(module.name(), module);
        break;
    }
}