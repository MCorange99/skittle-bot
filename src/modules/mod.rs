/*
* available modules
*/
pub mod types;

#[cfg(feature = "core")]
pub mod core;

#[cfg(feature = "moderation")]
pub mod moderation;
#[cfg(feature = "music")]
pub mod music;


use std::{collections::HashMap, fmt::Write};
use serenity::{model::{Permissions, prelude::Message}, prelude::Context};
use color_eyre::Result;
use types::SkittleModuleCommandFn;

use crate::db::models::UserRole;

use self::types::SkittleModuleEventManagerFn;

#[derive(Debug, Clone)]
pub struct SkittleModuleCommand {
    pub exec: SkittleModuleCommandFn,
    pub description: String,
    pub help: Vec<(String, String)>,
    pub dev_only: bool,
    pub required_user_roles: Vec<UserRole>,
    pub required_bot_permissions: Vec<Permissions>,
}

pub struct SkittleModuleCommandBuilder {
    inner: SkittleModuleCommand
}

impl SkittleModuleCommandBuilder {
    pub fn new(exec: SkittleModuleCommandFn) -> Self {
        Self {
            inner: SkittleModuleCommand { 
                exec,
                help: Vec::new(), 
                dev_only: false, 
                required_user_roles: Vec::new(), 
                required_bot_permissions: Vec::new(),
                description: String::new(),
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
    pub fn required_user_roles(&mut self, val: Vec<UserRole>) -> &mut Self {
        self.inner.required_user_roles = val;
        self
    }
    pub fn required_bot_permissions(&mut self, val: Vec<Permissions>) -> &mut Self {
        self.inner.required_bot_permissions = val;
        self
    }

    pub fn description(&mut self, desc: &str) -> &mut Self {
        self.inner.description = desc.to_string();
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
    commands: HashMap<String, SkittleModuleCommand>,
    event_handler: Option<SkittleModuleEventManagerFn>
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
            event_handler: None,
        }
    }

    /*
        Setters
     */
    pub fn set_description(&mut self, value: &str){
        self.description = Some(value.to_string());
    }

    pub fn set_version(&mut self, value: &str){
        self.version = Some(value.to_string());
    }

    pub fn set_author(&mut self, value: &str){
        self.author = Some(value.to_string());
    }

    pub fn register_command(&mut self, name: &str, comm: SkittleModuleCommand) {
        self.commands.insert(name.to_string(), comm);
    }

    pub fn register_event_handler(&mut self, handler: SkittleModuleEventManagerFn) {
        self.event_handler = Some(handler);
    }

    /*
        Getters
     */

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

    pub fn event_handler(&mut self) -> Option<SkittleModuleEventManagerFn> {
        self.event_handler
    }
}


pub struct Stdout {
    buf: String,
    msg: Message,
    ctx: Context
}

impl Stdout {
    pub fn new(msg: &Message, ctx: &Context) -> Self {
        Self {
            buf: String::new(),
            msg: msg.clone(),
            ctx: ctx.clone()
        }
    }

    pub async fn flush(&mut self) -> Result<()> {
        self.msg.reply_ping(&self.ctx.http, format!(
            "```sh\n{}```",
            self.buf
        )).await?;
        self.buf.clear();
        Ok(())
    }
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buf.write_str(s)
    }
}