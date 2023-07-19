use std::ffi::OsStr;

use crate::CoreData;
use anyhow::Result;

use libloading;
use skittle_bot_core::SkittleModule;

pub fn load(cd: &mut CoreData) -> Result<()> {
    unsafe {

        let modules = std::fs::read_dir("./modules")?;
        
        for module_file in modules {
            let module_file = module_file?;
            if !module_file.file_type()?.is_file() { continue; }
            if module_file.path().extension().unwrap_or_default().to_str().unwrap_or("") != "so" { 
                log::warn!("Non module file found in module folder: {:?}", module_file.path());
                continue;
            }
            
            let module = libloading::Library::new(module_file.path())?;
            let module_reg_fn = module.get::<fn() -> SkittleModule>(b"register")?;
            let mut module_registration = module_reg_fn();
            cd.modules.insert(module_registration.name(), module_registration);
        }
        
        Ok(())
    }
}