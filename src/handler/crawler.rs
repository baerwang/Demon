use headless_chrome::{Browser, LaunchOptions};

use crate::{common, model};

pub fn browse_wikipedia(
    config: model::task::TaskConfig,
    launch_options: LaunchOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let browser = Browser::new(launch_options)?;
    let locked_vec = browser.get_tabs().lock().unwrap();
    let tabs = &*locked_vec[1];
    tabs.close(true)?;
    drop(locked_vec);

    let random_ug = common::user_agent::random_user_agent();
    for item in &config.target {
        let tab = browser.new_tab()?;
        tab.enable_fetch(None, Some(true))?;
        tab.authenticate(config.username.clone(), config.password.clone())?;
        tab.set_user_agent(random_ug.as_str(), None, None).unwrap();
        tab.navigate_to(item)?;
        tab.set_extra_http_headers(
            config
                .headers
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect(),
        )
        .unwrap();
        let ug = tab
            .evaluate("window.navigator.userAgent", false)?
            .value
            .unwrap();
        assert_eq!(random_ug, ug);
        _ = tab.close(true);
    }

    Ok(())
}
