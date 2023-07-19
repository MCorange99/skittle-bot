mod commands;

use std::{collections::HashMap, future::Future, ops::Deref};
use anyhow::Result;
use serenity::prelude::Context;
use skittle_bot_core::{SkittleModule, SkittleModuleCommand};


#[no_mangle]
pub fn register() -> SkittleModule {
    let mut module = SkittleModule::register("core");
    module.set_author("mcorange");
    module.set_version("0.0.1");

    let ping = commands::ping::exec;

    module.register_command("ping", ping);


    module.clone()
}