use super::{ IchaError, IchaResult };
use std::{path, env, fs};
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    debug: bool,
    keys: Option<Keys>,
    server: Option<ServerConfig>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Keys {
    username: String,
    secret: String,
}
#[derive(Default, Serialize, Deserialize)]
pub struct ServerConfig {
    ip: String,
    port: Option<u16>
}

impl Config {

    /// Get or create default dir
    pub fn dir() -> Option<path::PathBuf> {
        let mut dir = dirs::config_dir()
            .or(dirs::home_dir())
            .or(dirs::data_dir())
            .unwrap_or_default();
        dir.push("icha");
        if dir.is_dir() {
            return Some(dir);
        } else if let Ok(_) = fs::create_dir(&dir) {
            return Some(dir);
        }
        return None;
    }

    /// Get current config at ~/.config/icha/config.toml
    /// or create default config 
    pub fn path() -> Option<path::PathBuf> {
        let dir = Self::dir().unwrap_or_default();
        let file = dir.join("config.toml");
        if file.is_file() {
            return Some(file);
        } else {
            let conf = toml::to_string(&Self::default())
                .expect("Could not serialize default conf");
            let _ = fs::write(&file, conf)
                .expect("Could not write config to file");
            return Some(file);
        }
    }

    pub fn from_file() -> IchaResult<Self> {
        let path = &Self::path().unwrap_or_default();
        let conf = fs::read_to_string(&path)?;
        let conf = toml::from_str(&conf)
            .expect("Could ot read config fro str");
        return Ok(conf);

    }
}
impl ToString for Config {
    fn to_string(&self) -> String {
        toml::to_string(&self)
            .expect("Could not read toml from string")
    }
}
