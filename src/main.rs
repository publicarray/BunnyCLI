#![allow(unused)]
#![forbid(unsafe_code)]
extern crate bunnycdn;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate keyring;
extern crate rpassword;
extern crate simplelog;
use anyhow::Result;
use bunnycdn::*;
use simplelog::*;
pub mod config;
use crate::config::*;
pub mod cli;
use crate::cli::*;

use tokio::runtime::{Builder, Runtime};

fn rt() -> Result<Runtime> {
    Ok(Builder::new().basic_scheduler().enable_all().build()?)
}

const APP_NAME: &str = "bunnycli-storage";

fn main() -> Result<()> {
    let default_config_file = get_default_config_file()?;
    let cli = create_cli(default_config_file.as_str());

    let log_level = match cli.occurrences_of("v") {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        3 | _ => LevelFilter::Trace,
    };

    if let Err(_) = TermLogger::init(log_level, simplelog::Config::default(), TerminalMode::Mixed) {
        SimpleLogger::init(log_level, simplelog::Config::default()).expect("No logger should be already set")
    }

    let config_file = cli.value_of("config").unwrap_or("bunnycli.toml");
    debug!("Value for config_file: {}", config_file);

    let mut settings = load_config(&config_file)?;
    let storage_zone = settings.storage_zone();
    trace!("The storage_zone is '{:?}'", storage_zone);

    let mut rt = rt()?;

    if let Some(storagecli) = cli.subcommand_matches("storage") {
        if storagecli.is_present("login") {
            let storage_zone_name = storagecli.value_of("login").unwrap();
            let api_key = rpassword::read_password_from_tty(Some("Enter your Storage API Key: ")).unwrap();

            settings.set_storage_zone(storage_zone_name, api_key.trim())?;
            settings.save_config(config_file)?;
            println!("{}", "Config Saved!");
        } else if storagecli.is_present("upload") {
            let args: Vec<&str> = storagecli.values_of("upload").unwrap().collect();
            let (file, url) = (args[0], args[1]);
            debug!("upload {} {}", file, url);
            let response = rt.block_on(storage_zone.upload_file(file, url))?;
            trace!("storagecli: {:?}", response);
            response.print();
        } else if storagecli.is_present("download") {
            let args: Vec<&str> = storagecli.values_of("download").unwrap().collect();
            let (file, url) = (args[0], args[1]);
            debug!("download {} {}", file, url);
            let response = rt.block_on(storage_zone.download_file(file, url))?;
            trace!("storagecli: {:?}", response);
            response.print();
        } else if storagecli.is_present("remove") {
            let url = storagecli.value_of("remove").unwrap();
            debug!("remove {}", url);
            let response = rt.block_on(storage_zone.delete(url))?;
            trace!("storagecli: {:?}", response);
            response.print();
        } else if storagecli.is_present("info") {
            let url = storagecli.value_of("info").unwrap();
            debug!("info {}", url);
            let response = rt.block_on(storage_zone.get_objects(url))?;
            trace!("storagecli: {:?}", response);
            response.print();
        } else {
            error!("Missing flags");
        }
    }
    Ok(())
}
