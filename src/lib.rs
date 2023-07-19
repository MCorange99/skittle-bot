use std::{future::Future, collections::HashMap};
use anyhow::Result;
use serenity::{prelude::Context, model::prelude::Message};



pub type SkittleModuleCommand = fn(ctx: Context, msg: Message, args: Vec<String>) -> dyn Future<Output = Result<()>>;

#[derive(Debug, Clone)]
pub struct SkittleModule {
    name: String,
    description: Option<String>,
    version: Option<String>,
    author: Option<String>,
    commands: HashMap<String, SkittleModuleCommand>
    // TODO: add events
}

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