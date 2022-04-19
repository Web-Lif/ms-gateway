use serde::Deserialize;


#[derive(Deserialize, Clone)]
pub struct SQLXConfig {
    pub url: String,
    pub max_connections: Option<u32>,
    pub min_connections: Option<u32>
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub addr: String,
    pub port: Option<u16>,
    pub sqlx: SQLXConfig
}

pub fn read_config () -> Config {
    let content = std::fs::read_to_string("resources/application.toml").unwrap();
    let config: Config = toml::from_str(&content).unwrap();
    config
}