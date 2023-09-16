use color_eyre::Result;
use toml;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct CoreConfig {
    pub prefix: String,
    pub database_url: String,
    pub write_logs_to_file: bool,
    pub debug: bool,
    pub secrets: CoreConfigSecrets,
    pub users: CoreConfigUsers,
    pub roles: CoreConfigRoles,
    pub modules: CoreConfigModules
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct CoreConfigSecrets {
    pub token: String,
    pub client_id: String,
    pub client_secret: String,
    pub invite: String,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct CoreConfigUsers {
    pub developers: Vec<String>
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct CoreConfigRoles {
    pub administrator: Vec<String>,
    pub moderator: Vec<String>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct CoreConfigModules {
    pub disabled_modules: Vec<String>
}



pub fn get_core_config() -> Result<CoreConfig> {
    let cfg: CoreConfig = toml::from_str(std::fs::read_to_string("./config/core.toml")?.as_str())?;
    Ok(cfg)
}
#[allow(dead_code)]
pub fn set_core_config(cfg: CoreConfig) -> Result<()> {
    Ok(std::fs::write("./config/core.toml", toml::to_string_pretty(&cfg)?)?)
}