use headless_chrome::{Browser, LaunchOptions};

pub fn browse_wikipedia(launch_options: LaunchOptions) -> Result<(), Box<dyn std::error::Error>> {
    let browser = Browser::new(launch_options)?;
    let tab = browser.new_tab()?;
    tab.navigate_to("https://example.com")?;
    let h1 = tab.wait_for_xpath("/html/body/div/h1")?;
    assert_eq!(h1.get_inner_text().unwrap().as_str(), "Example Domain");
    Ok(())
}
