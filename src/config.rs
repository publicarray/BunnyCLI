use anyhow::{bail, Context, Result};
use bunnycdn::*;
use serde::{Deserialize, Serialize};
use std::fs;
const APP_NAME: &str = "bunnycli-storage";
const SERVER_URL: &str = "https://storage.bunnycdn.com";

pub fn get_default_config_file() -> Result<String> {
    let mut home_path = match dirs::home_dir() {
        Some(home_path) => home_path,
        None => bail!("Could not get Home path '~'"),
    };
    home_path.push(std::path::Path::new(".config/bunnycli.tml"));
    Ok(home_path.into_os_string().into_string().unwrap())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub storage_zone: Option<StorageZone>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StorageZone {
    pub name: String,
    pub api_endpoint: String,
}

impl Config {
    pub fn save_config(&self, config_file: &str) -> Result<()> {
        let toml_str = toml::to_string(self).context("Failed to convert config struct to toml string")?;
        fs::write(config_file, toml_str).with_context(|| format!("Failed to write config file: {}", config_file))?;
        Ok(())
    }

    pub fn storage_zone(&mut self) -> storage::StorageZone {
        if let Some(conf_storage_zone) = &self.storage_zone.as_ref() {
            let keyring = keyring::Keyring::new(APP_NAME, &conf_storage_zone.name);
            let storage_api_key = keyring.get_password().unwrap_or_else(|err| {
                error!("{}", err);
                String::new()
            });
            let mut storage_zone = storage::StorageZone::new(conf_storage_zone.name.clone(), storage_api_key);
            if conf_storage_zone.api_endpoint.len() > 0 {
                storage_zone.set_api_endpoint(&conf_storage_zone.api_endpoint);
            }
            storage_zone
        } else {
            warn!("no storage_zone in config file");
            storage::StorageZone::new("".to_string(), "".to_string())
        }
    }

    pub fn set_storage_zone(&mut self, zone_name: &str, api_key: &str) -> Result<()> {
        let keyring = keyring::Keyring::new(APP_NAME, &zone_name);
        keyring.set_password(&api_key);
        self.storage_zone = Some(StorageZone{name: zone_name.to_string(), api_endpoint: SERVER_URL.to_string()});
        Ok(())
    }
}

pub fn load_config(config_file: &str) -> Result<Config> {
    let mut config: Config = Config { storage_zone: None };
    if std::path::Path::new(config_file).exists() {
        let toml_str =
            fs::read_to_string(config_file).with_context(|| format!("Failed to read config file: {}", config_file))?;
        config = toml::from_str(&toml_str).with_context(|| format!("Failed to read config file: {}", config_file))?;
        trace!("{:#?}", config);
    } else {
        fs::File::create(config_file)?;
        info!("Config file created");
    }
    Ok(config)
    // settings.merge(config::Environment::with_prefix("BUNNY")).unwrap();
}
