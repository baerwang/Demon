use headless_chrome::{Browser, LaunchOptions};

use crate::common;

pub fn browse_wikipedia(launch_options: LaunchOptions) -> Result<(), Box<dyn std::error::Error>> {
    let browser = Browser::new(launch_options)?;
    let tab = browser.new_tab()?;
    let random_ug = common::user_agent::random_user_agent();
    tab.set_user_agent(random_ug.as_str(), None, None).unwrap();
    tab.navigate_to("https://example.com")?;
    let h1 = tab.wait_for_xpath("/html/body/div/h1")?;
    assert_eq!(h1.get_inner_text().unwrap().as_str(), "Example Domain");
    let ug = tab
        .evaluate("window.navigator.userAgent", false)?
        .value
        .unwrap();
    assert_eq!(random_ug, ug);
    Ok(())
}
