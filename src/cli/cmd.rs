use std::collections::HashMap;

use anyhow::anyhow;
use headless_chrome::browser::default_executable;
use headless_chrome::LaunchOptions;

use crate::handler::crawler;
use crate::model;

pub fn demon_args() -> impl IntoIterator<Item = impl Into<clap::Arg>> {
    [
        clap::Arg::new("target")
            .long("target")
            .alias("target")
            .action(clap::ArgAction::Set)
            .num_args(0..)
            .help("Custom Http Headers"),
        clap::Arg::new("custom-headers")
            .long("custom-headers")
            .alias("custom-headers")
            .action(clap::ArgAction::Set)
            .num_args(0..)
            .help("Custom Http Headers"),
    ]
}

pub fn chromium_args() -> impl IntoIterator<Item = impl Into<clap::Arg>> {
    [
        clap::Arg::new("path").long("path").alias("path")
            .help("Path for Chrome or Chromium."),
        clap::Arg::new("headless").long("headless").alias("headless")
            .default_value("true")
            .help("Determines whether to run headless version of the browser. Defaults to true."),
        clap::Arg::new("proxy").long("proxy").alias("proxy")
            .help("Setup the proxy server for headless chrome instance"),
        clap::Arg::new("sandbox").long("sandbox").alias("sandbox")
            .default_value("true")
            .help("Determines whether to run the browser with a sandbox."),
        clap::Arg::new("ignore_certificate_errors").long("ignore_certificate_errors")
            .alias("ignore_certificate_errors")
            .default_value("true")
            .help("Determines whether SSL certificates should be verified."),
        clap::Arg::new("user_data_dir").long("user_data_dir").alias("user_data_dir")
            .help("User Data (Profile) to use If unspecified, a new temp directory is created and used on every launch."),
    ]
}

pub fn web_args() -> impl IntoIterator<Item = impl Into<clap::Arg>> {
    [clap::Arg::new("port")
        .long("port")
        .alias("port")
        .default_value("9999")
        .help("Start Port")]
}

pub fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let app = clap::Command::new("demon")
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .args(demon_args())
        .subcommands(&[
            clap::Command::new("chromium")
                .args(chromium_args())
                .about("chromium setup"),
            clap::Command::new("web").args(web_args()).about("web UI"),
        ])
        .get_matches();

    let headers: HashMap<_, _> = app
        .get_many::<String>("custom-headers")
        .unwrap_or_default()
        .map(|pair| {
            let mut iter = pair.split(':');
            let key = iter.next().expect("No key found");
            let value = iter.next().expect("No value found");
            (key.to_string(), value.to_string())
        })
        .collect();

    let chromium_path = Some(default_executable().map_err(|e| anyhow!(e))?);

    let target = app
        .get_many::<String>("target")
        .expect("target not allow empty")
        .map(|s| s.to_string())
        .collect();

    let config = model::task::TaskConfig {
        target,
        headers,
        robots: false,
        range: 0,
        repeat: 0,
    };

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("INFO"));

    let buf = std::env::current_dir()
        .unwrap()
        .join("files/user_agent.toml");
    std::env::set_var("user_agent", buf);

    if app.subcommand().is_none() {
        let launch_options = LaunchOptions::default_builder()
            .path(chromium_path)
            .build()?;
        return crawler::browse_wikipedia(config, launch_options);
    }

    let (name, command) = app.subcommand().unwrap();
    match name {
        "chromium" => {
            let path = match command.get_one::<String>("path") {
                Some(h) => Some(std::path::PathBuf::from(h.parse::<String>().unwrap())),
                None => chromium_path,
            };

            let proxy = command.get_one::<String>("proxy").map(|h| h.as_str());
            let headless = default_bool(command.clone(), "headless");
            let sandbox = default_bool(command.clone(), "sandbox");
            let ignore_certificate_errors =
                default_bool(command.clone(), "ignore_certificate_errors");
            let user_data_dir = command
                .get_one::<String>("user_data_dir")
                .map(|h| std::path::PathBuf::from(h.parse::<String>().unwrap()));

            let launch_options = LaunchOptions::default_builder()
                .path(path)
                .proxy_server(proxy)
                .headless(headless)
                .sandbox(sandbox)
                .ignore_certificate_errors(ignore_certificate_errors)
                .user_data_dir(user_data_dir)
                .build()?;

            crawler::browse_wikipedia(config, launch_options)
        }
        _ => {
            panic!("The current feature is not implemented or {name} does not exist")
        }
    }
}

fn default_bool(command: clap::ArgMatches, key: &str) -> bool {
    command
        .get_one::<String>(key)
        .map(|h| h.parse::<bool>().unwrap_or(true))
        .unwrap_or(true)
}
