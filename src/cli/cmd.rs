use anyhow::anyhow;
use headless_chrome::browser::default_executable;
use headless_chrome::LaunchOptions;

use crate::handler::crawler;

pub fn chromium_args() -> impl IntoIterator<Item = impl Into<clap::Arg>> {
    [
        clap::Arg::new("path").long("chromium-path").alias("path")
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
        .subcommands(&[
            clap::Command::new("chromium")
                .args(chromium_args())
                .about("chromium setup"),
            clap::Command::new("web").args(web_args()).about("web UI"),
        ])
        .get_matches();

    let buf = std::env::current_dir()
        .unwrap()
        .join("files/user_agent.toml");
    std::env::set_var("user_agent", buf);

    if app.subcommand().is_none() {
        return crawler::browse_wikipedia(LaunchOptions::default());
    }

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("INFO"));

    let (name, command) = app.subcommand().unwrap();
    match name {
        "chromium" => {
            let path = match command.get_one::<String>("path") {
                Some(h) => Some(std::path::PathBuf::from(h.parse::<String>().unwrap())),
                None => Some(default_executable().map_err(|e| anyhow!(e))?),
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

            crawler::browse_wikipedia(launch_options)
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
