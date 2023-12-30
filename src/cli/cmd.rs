use std::collections::HashMap;

use anyhow::anyhow;
use clap::Parser;
use headless_chrome::browser::default_executable;
use headless_chrome::{browser, LaunchOptions};

use crate::cli::args;
use crate::handler::crawler;
use crate::{common, model};

pub fn cli() -> Result<(), Box<dyn std::error::Error>> {
    let app = args::CLi::parse();
    let headers: HashMap<_, _> = app
        .custom_headers
        .iter()
        .map(|pair| {
            let mut iter = pair.split(':');
            let key = iter.next().expect("No key found");
            let value = iter.next().expect("No value found");
            (key.to_string(), value.to_string())
        })
        .collect();

    let config = model::task::TaskConfig {
        target: app.target,
        headers,
        username: app.username,
        password: app.password,
        robots: app.robots,
        range: 0,
        repeat: 0,
    };

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("INFO"));

    common::load("user_agent", "files/user_agent.toml");
    common::load("form", "files/form.toml");

    let options = browser::FetcherOptions::default().with_allow_download(false);

    let opt = app.opt.unwrap_or_default();
    match opt {
        args::Opt::Chromium(c) => {
            let path = Some(
                c.path
                    .unwrap_or(default_executable().map_err(|e| anyhow!(e))?),
            );
            let proxy = Some(c.proxy.as_deref().unwrap_or_default());
            let launch_options = LaunchOptions::default_builder()
                .path(path)
                .headless(c.headless)
                .sandbox(c.sandbox)
                .proxy_server(proxy)
                .ignore_certificate_errors(c.ignore_certificate_errors)
                .user_data_dir(c.user_data_dir)
                .fetcher_options(options)
                .build()?;

            crawler::browse_wikipedia(config, launch_options)
        }
    }
}
