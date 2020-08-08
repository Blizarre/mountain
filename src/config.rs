use std::fs;
extern crate serde;
extern crate serde_derive;

use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct ScreenConfig {
    pub width: i32,
    pub height: i32,
}

#[derive(Deserialize)]
pub struct MapConfig {
    pub texture: String,
    pub heightmap: String,
}

#[derive(Deserialize)]
pub struct PlayerConfig {
    pub height: i32,
    pub speed: i32,
    pub sensitivity_x: f32,
    pub sensitivity_y: f32,
}

#[derive(Deserialize)]
pub struct RendererConfig {
    pub fog: bool,
    pub fog_start: i32,
    pub distance_max: i32,
    pub enable_hm_filtering: bool,
}

#[derive(Deserialize)]
pub struct Config {
    pub renderer: RendererConfig,
    pub screen: ScreenConfig,
    pub map: MapConfig,
    pub player: PlayerConfig,
}

pub struct ConfigError {
    pub message: String,
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError {
            message: err.to_string(),
        }
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError {
            message: err.to_string(),
        }
    }
}

impl Config {
    pub fn from_config(file_path: &str) -> Result<Config, ConfigError> {
        let config_text = fs::read_to_string(file_path)?;
        toml::from_str::<Config>(config_text.as_str()).map_err(|e| e.into())
    }
}
