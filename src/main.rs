#![allow(unused)]
extern crate bunnycdn;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate simplelog;
use anyhow::{anyhow, bail, Context, Result};
use bunnycdn::*;
use clap::{App, Arg, SubCommand};
use simplelog::*;
use std::collections::HashMap;
use std::fs;
use std::io;

use tokio::runtime::{Builder, Runtime};

fn rt() -> Result<Runtime> {
    Ok(Builder::new().basic_scheduler().enable_all().build()?)
}

fn get_default_config_file() -> Result<String> {
    let mut home_dir = String::new();
    let mut home_path = match dirs::home_dir() {
        Some(home_path) => home_path,
        None => bail!("Could not get Home path '~'"),
    };
    home_path.push(std::path::Path::new(".config/bunnycli.tml"));
    Ok(home_path.into_os_string().into_string().unwrap())
}

use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
struct Config {
    storage_zone: Option<storage::StorageZone>,
}

fn load_config(config_file: &str) -> Result<Config> {
    let mut config: Config = Config { storage_zone: None };
    if std::path::Path::new(config_file).exists() {
        let toml_str =
            fs::read_to_string(config_file).with_context(|| format!("Failed to read config file: {}", config_file))?;
        config = toml::from_str(&toml_str).with_context(|| format!("Failed to read config file: {}", config_file))?;
        trace!("{:#?}", config);
    } else {
        return Err(anyhow!("Config file not found: {}", config_file));
    }
    Ok(config)
    // settings.merge(config::Environment::with_prefix("BUNNY")).unwrap();
}

fn save_config(config_file: &str, config: &Config) -> Result<()> {
    let toml_str = toml::to_string(config).context("Failed to convert config struct to toml string")?;
    fs::write(config_file, toml_str).with_context(|| format!("Failed to write config file: {}", config_file))?;
    Ok(())
}

fn main() -> Result<()> {
    let default_config_file = get_default_config_file()?;
    let cli = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .default_value(default_config_file.as_str())
             .help("Sets a custom config file")
             .takes_value(true))
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("storage")
                .about("Interact with BunnyCDN Storage Zones")
                .arg(
                    Arg::with_name("login")
                        .long("login")
                        .short("l")
                        .value_name("STORAGE ZONE")
                        .help("Save Login Credentials"),
                )
                .arg(
                    Arg::with_name("upload")
                        .long("upload")
                        .short("u")
                        .value_name("FILE")
                        .value_name("URL")
                        .help("Upload a file to a storage zone based on the URL path. If the directory tree does not exist, it will be created automatically."),
                )
                .arg(
                    Arg::with_name("download")
                        .long("download")
                        .short("d")
                        .value_name("FILE")
                        .value_name("URL")
                        .help("Download a file from your Storage Zone."),
                )
                .arg(
                    Arg::with_name("remove")
                        .long("remove")
                        .short("r")
                        .visible_alias("delete")
                        .value_name("URL")
                        .help("Delete an object from the storage zone. In case the object is a directory all the data in it will be recursively deleted as well.
"),
                )
                .arg(
                    Arg::with_name("directory")
                        .long("directory")
                        .short("i")
                        .value_name("URL")
                        .help("Retrieve a list of files and directories located in the given directory."),
                ),
        )
        .get_matches();

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
    let mut storagezone = storage::StorageZone::new(String::new(), String::new());
    let mut settings = load_config(&config_file)?;
    if let Some(zone) = settings.storage_zone {
        storagezone = zone
    }

    let mut rt = rt()?;

    if let Some(cli) = cli.subcommand_matches("storage") {
        if cli.is_present("login") {
            let storage_zone_name = cli.value_of("login").unwrap();
            println!("Enter your API Key:");
            let mut api_key = String::new();
            io::stdin().read_line(&mut api_key)?;
            // let stdin = io::stdin();
            // for line in stdin.lock().lines() {
            //     api_key = line.unwrap();
            //     break
            // }
            // ask for api key
            let storage_zone = storage::StorageZone::new(storage_zone_name.to_string(), api_key.trim().to_string());
            debug!("{:?}", storage_zone);
            settings.storage_zone = Some(storage_zone);
            save_config(config_file, &settings)?;
        // save_conf(settings)
        // settings.merge(config::Config::try_from(&storage_zone1).unwrap());
        // settings.refresh();
        // settings
        //     .set("storage.zone_name", storage_zone.to_string())
        //     .unwrap()
        //     .set("storage.api_key", api_key.trim().to_string())
        //     .unwrap();

        // println!("{:?}", settings.try_into::<HashMap<String, String>>().unwrap());
        // ask for api key
        // save storage_zone,key
        // ~/.config/bunnycdn.toml
        } else if cli.is_present("upload") {
            let args: Vec<&str> = cli.values_of("upload").unwrap().collect();
            let (file, url) = (args[0], args[1]);
            debug!("upload {} {}", file, url);
            let response = rt.block_on(storagezone.upload_file(file, url)).unwrap();
            trace!("cli: {:?}", response);
            response.print();
        } else if cli.is_present("download") {
            let args: Vec<&str> = cli.values_of("download").unwrap().collect();
            let (file, url) = (args[0], args[1]);
            debug!("download {} {}", file, url);
            let response = rt.block_on(storagezone.download_file(file, url)).unwrap();
            trace!("cli: {:?}", response);
            response.print();
        } else if cli.is_present("remove") {
            let url = cli.value_of("remove").unwrap();
            debug!("remove {}", url);
            let response = rt.block_on(storagezone.delete(url))?;
            trace!("cli: {:?}", response);
            response.print();
        } else if cli.is_present("directory") {
            let url = cli.value_of("directory").unwrap();
            debug!("directory {}", url);
            let response = rt.block_on(storagezone.get_objects(url)).unwrap();
            trace!("cli: {:?}", response);
            response.print();
        } else {
            error!("Missing flags");
            println!("{}", cli.usage());
        }
    } else {
        println!("{}", cli.usage());
    }
    Ok(())
}
