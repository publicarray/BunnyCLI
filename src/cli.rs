use clap::{App, Arg, AppSettings};

pub fn create_cli(default_config_file: &str) -> clap::ArgMatches {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("config")
             .short('c')
             .long("config")
             .value_name("FILE")
             .default_value(default_config_file)
             .about("Sets a custom config file")
             .takes_value(true))
        .arg(Arg::with_name("v")
                .short('v')
                .multiple(true)
                .about("Sets the level of verbosity"))
        .subcommand(App::new("pullzone")
                .about("Interact with BunnyCDN Pull Zones")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .arg(Arg::with_name("list")
                        .long("list")
                        .short('l')
                        .about("Returns the list of all the Pull Zones in the user's account"))
                .arg(Arg::with_name("create")
                        .long("create")
                        .short('c')
                        .value_name("NAME")
                        .value_name("ORIGIN")
                        .about("Create a new Pull Zone with the given name and Origin URL"))
                .arg(Arg::with_name("details")
                        .long("details")
                        .short('d')
                        .value_name("ID")
                        .about("Returns the Pull Zone details for the zone with the given ID"))
                .arg(Arg::with_name("Update")
                        .long("update")
                        .short('u')
                        .value_name("ID")
                        .about("Updates the pull zone configuration. All the fields are required to succeed"))
                .arg(Arg::with_name("remove")
                        .long("remove")
                        .short('r')
                        .visible_alias("delete")
                        .value_name("ID")
                        .about("Deletes the Pull Zone with the given ID"))
                .arg(Arg::with_name("purge cache")
                        .long("purge-cache")
                        .short('p')
                        .value_name("ID")
                        .about("Purges the full cache for the given Pull Zone"))
                .arg(Arg::with_name("hostname")
                        .long("hostname")
                        .value_name("ID")
                        .value_name("HOSTNAME")
                        .about("Register a custom hostname to a Pull Zone"))
                .arg(Arg::with_name("remove-hostname")
                        .long("remove-hostname")
                        .value_name("ID")
                        .about("Delete a custom hostname from a Pull Zone"))
                .arg(Arg::with_name("force-ssl")
                        .long("force-ssl")
                        .value_name("ID")
                        .about("The endpoint used to enable or disable the Force SSL setting for a given pull zone"))
                .arg(Arg::with_name("free-ssl-cert")
                        .long("free-ssl-cert")
                        .value_name("ID")
                        .about("Loads a FREE SSL Certificate to the domain provided by Let's Encrypt"))
                .arg(Arg::with_name("install-ssl-cert")
                        .long("install-ssl-cert")
                        .value_name("ID")
                        .about("Adds a custom certificate to the given Pull Zone"))
                .arg(Arg::with_name("block-ip")
                        .long("block-ip")
                        .value_name("ID")
                        .about("Add an IP to the list of blocked IPs that are not allowed to access the zone"))
                .arg(Arg::with_name("remove-block")
                        .long("remove-block")
                        .value_name("ID")
                        .about("Remove an IP from the list of blocked IPs that are not allowed to access the zone"))
                .subcommand(App::new("edgerule")
                    .about("Manipulate Edge Rules")
                    .setting(AppSettings::SubcommandRequiredElseHelp)
                    .arg(Arg::with_name("add-update")
                        .long("create")
                        .visible_alias("update")
                        .short('c')
                        .value_name("PULLZONE_ID")
                        .value_name("EDGERULE_ID")
                        .about("Add or update the edge rule."))
                    .arg(Arg::with_name("remove")
                        .long("remove")
                        .visible_alias("delete")
                        .short('r')
                        .value_name("PULLZONE_ID")
                        .value_name("EDGERULE_ID")
                        .about("Delete the edge rule"))
                )
        )
        .subcommand(App::new("storage")
                .about("Interact with BunnyCDN Storage Zones")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .arg(Arg::with_name("login")
                        .long("login")
                        .short('l')
                        .value_name("STORAGE ZONE")
                        .about("Save Login Credentials"))
                .arg(Arg::with_name("upload")
                        .long("upload")
                        .short('u')
                        .value_name("FILE")
                        .value_name("URL")
                        .about("Upload a file to a storage zone based on the URL path. If the directory tree does not exist, it will be created automatically."))
                .arg(Arg::with_name("download")
                        .long("download")
                        .short('d')
                        .value_name("FILE")
                        .value_name("URL")
                        .about("Download a file from your Storage Zone."))
                .arg(Arg::with_name("remove")
                        .long("remove")
                        .short('r')
                        .visible_alias("delete")
                        .value_name("URL")
                        .about("Delete an object from the storage zone. In case the object is a directory all the data in it will be recursively deleted as well."))
                .arg(Arg::with_name("directory")
                        .long("directory")
                        .short('i')
                        .value_name("URL")
                        .about("Retrieve a list of files and directories located in the given directory."))
                .arg(Arg::with_name("logout")
                        .long("logout")
                        .value_name("STORAGE ZONE")
                        .about("Remove credentials."))
        ).get_matches()
}
