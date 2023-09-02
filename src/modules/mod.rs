/*
* available modules
*/

pub mod core;
pub mod moderation;
pub mod music;



/*
* Types
*/

use std::collections::HashMap;
use color_eyre::Result;
use futures::future::BoxFuture;
use serenity::{prelude::Context, model::{prelude::Message, Permissions}};



pub type SkittleModuleCommandExec = fn(ctx: Context, msg: Message, args: Vec<String>) -> BoxFuture<'static, Result<()>>;

#[derive(Debug, Clone)]
pub struct SkittleModuleCommand {
    pub exec: SkittleModuleCommandExec,
    pub help: Vec<(String, String)>,
    pub dev_only: bool,
    pub required_user_permissions: Vec<Permissions>,
    pub required_bot_permissions: Vec<Permissions>
}

pub struct SkittleModuleCommandBuilder {
    inner: SkittleModuleCommand
}

impl SkittleModuleCommandBuilder {
    pub fn new(exec: SkittleModuleCommandExec) -> Self {
        Self {
            inner: SkittleModuleCommand { 
                exec,
                help: Vec::new(), 
                dev_only: false, 
                required_user_permissions: Vec::new(), 
                required_bot_permissions: Vec::new()
            }
        }
    }

    pub fn help(&mut self, val: Vec<(&str, &str)>) -> &mut Self {
        self.inner.help = val.into_iter().map(|v| (v.0.to_string(), v.1.to_string())).collect();
        self
    }
    pub fn dev_only(&mut self, val: bool) -> &mut Self {
        self.inner.dev_only = val;
        self
    }
    pub fn required_user_permissions(&mut self, val: Vec<Permissions>) -> &mut Self {
        self.inner.required_user_permissions = val;
        self
    }
    pub fn required_bot_permissions(&mut self, val: Vec<Permissions>) -> &mut Self {
        self.inner.required_bot_permissions = val;
        self
    }

    pub fn build(&mut self) -> SkittleModuleCommand {
        self.inner.clone()
    }
}


#[derive(Debug, Clone)]
pub struct SkittleModule {
    name: String,
    description: Option<String>,
    version: Option<String>,
    author: Option<String>,
    commands: HashMap<String, SkittleModuleCommand>
    // TODO: add events
}

#[allow(dead_code)]
impl SkittleModule {
    pub fn register(name: &str) -> Self {
        Self {
            name: name.to_string(),
            description: None,
            version: None,
            author: None,
            commands: HashMap::new(),
        }
    }

    pub fn set_description(&mut self, value: &str){
        self.description = Some(value.to_string());
    }

    pub fn set_version(&mut self, value: &str){
        self.version = Some(value.to_string());
    }

    pub fn set_author(&mut self, value: &str){
        self.author = Some(value.to_string());
    }

    pub fn name(&mut self) -> String {
        self.name.clone()
    }

    pub fn description(&mut self) -> String {
        self.description.clone().unwrap_or("No description provided".to_string())
    }

    pub fn version(&mut self) -> String {
        self.version.clone().unwrap_or("No version provided".to_string())
    }

    pub fn author(&mut self) -> String {
        self.author.clone().unwrap_or("No author provided".to_string())
    }

    pub fn commands(&mut self) -> HashMap<String, SkittleModuleCommand> {
        self.commands.clone()
    }

    pub fn register_command(&mut self, name: &str, comm: SkittleModuleCommand) {
        self.commands.insert(name.to_string(), comm);
    }
}