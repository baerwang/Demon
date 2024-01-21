use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::sync::Arc;

use anyhow::anyhow;
use clap::Parser;
use crossbeam::channel::unbounded;
use crossbeam::channel::Sender;
use headless_chrome::browser::default_executable;
use headless_chrome::{browser, Browser, LaunchOptions};

use crate::channel::scope;
use crate::cli::args;
use crate::handler::crawler;
use crate::handler::robots::robots;
use crate::handler::sitemap::sitemap;
use crate::{channel, common, handler, model};

pub async fn cli() -> Result<(), Box<dyn std::error::Error>> {
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

    let scan_factory =
        handler::scan_policy::factory("", url::Url::parse(app.target[0].as_str()).unwrap(), None);

    let duplicate_factory = handler::duplicate::factory("");

    let config = model::task::TaskConfig {
        headers,
        username: app.username,
        password: app.password,
        robots: app.robots,
    };

    env_logger::init_from_env(env_logger::Env::new().default_filter_or(app.log_level));

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

            let args = vec![
                OsStr::new("--disable-web-security=true"),
                OsStr::new("--disable-xss-auditor=true"),
                OsStr::new("--disable-setuid-sandbox=true"),
                OsStr::new("--allow-running-insecure-content=true"),
                OsStr::new("--disable-webgl=true"),
                OsStr::new("--disable-popup-blocking=true"),
                OsStr::new("--block-new-web-contents=true"),
                OsStr::new("--blink-settings=imagesEnabled=false"),
            ];

            let proxy = Some(c.proxy.as_deref().unwrap_or_default());
            let launch_options = LaunchOptions::default_builder()
                .path(path)
                .headless(c.headless)
                .sandbox(c.sandbox)
                .proxy_server(proxy)
                .ignore_certificate_errors(c.ignore_certificate_errors)
                .user_data_dir(c.user_data_dir)
                .fetcher_options(options)
                .disable_default_args(true)
                .args(args)
                .build()?;

            let (tx, rcv) = unbounded();
            for url in app.target.clone() {
                scope(tx.clone(), url)
            }

            if app.robots {
                let root = Arc::new(app.target[0].clone());

                let robot_clone = Arc::clone(&root);
                if let Ok(t) = robots(robot_clone.to_string()).await {
                    gather(t, tx.clone())
                }

                let sitemap_clone = Arc::clone(&root);
                if let Ok(t) = sitemap(sitemap_clone.to_string()).await {
                    gather(t, tx.clone())
                }
            }

            // drop(tx);

            let browser = Browser::new(launch_options)?;
            let mut state = channel::GlobalState::new(
                tx.clone(),
                Arc::new(app.target[0].clone()).clone().to_string(),
                browser,
                scan_factory,
                duplicate_factory,
                config,
            );

            for url in rcv.iter() {
                if state.rx_store.insert(url.clone()) {
                    _ = crawler::tasks(url.clone().as_str(), tx.clone(), &mut state).await;
                }
            }
            Ok(())
        }
    }
}

fn gather(t: HashSet<String>, tx: Sender<String>) {
    for url in t {
        scope(tx.clone(), url)
    }
}
