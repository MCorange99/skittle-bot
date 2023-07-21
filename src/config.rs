use anyhow::Result;
use toml;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct CoreConfig {
    pub prefix: String,
    pub token: String,
    pub client_id: String,
    pub client_secret: String,
    pub invite: String
}



pub fn get_core_config() -> Result<CoreConfig> {
    let cfg: CoreConfig = toml::from_str(std::fs::read_to_string("./config/core.toml")?.as_str())?;
    Ok(cfg)
}
#[allow(dead_code)]
pub fn set_core_config(cfg: CoreConfig) -> Result<()> {
    Ok(std::fs::write("", toml::to_string_pretty(&cfg)?)?)
}