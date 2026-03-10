use std::fs;
use std::path;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub base: String,
    pub css: String,
    pub from_lore: String,
    pub to_html: String,
}

pub fn load_from_config() -> Config {
    let config_path = path::Path::new("wein.toml");
    let config_string = fs::read_to_string(config_path);
    let config: Config = toml::from_str(
        config_string
            .unwrap()
            .as_str()
    ).unwrap();
    config
}